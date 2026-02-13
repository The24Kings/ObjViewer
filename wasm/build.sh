#!/usr/bin/env bash
set -euo pipefail

# Build script for the WASM target.
#
# Usage:
#   ./wasm/build.sh              # dev build
#   ./wasm/build.sh --release    # release build
#
# Prerequisites (one-time):
#   rustup target add wasm32-unknown-unknown
#   cargo install -f wasm-bindgen-cli --version 0.2.105
#   cargo install -f wasm-tools
#   Emscripten SDK on PATH (for provider build)
#
# Steps:
#   1) Generate WASM bindings       (xtask wasm-bindgen)
#   2) Build cimgui provider        (xtask build-cimgui-provider)
#   3) Build our WASM crate         (wasm-pack)
#   4) Patch wasm binary            (import shared memory from env)
#   5) Patch JS glue                (inject shared memory into imports)
#   6) Copy provider files to pkg/

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
XTASK_DIR="$ROOT/vendor/dear-imgui-rs"
PKG="$ROOT/wasm/pkg"
WASM_BIN="$PKG/wasm_bg.wasm"
JS_GLUE="$PKG/wasm.js"

# Memory sizes must match provider (INITIAL_MEMORY=134217728 = 2048 pages of 64KiB)
INIT_PAGES=2048
MAX_PAGES=32768

PROFILE="--dev"
if [[ "${1:-}" == "--release" ]]; then
    PROFILE="--release"
fi

# ---------- Step 1: Generate pregenerated WASM bindings ----------
echo "==> [1/6] Generating WASM bindings (imgui-sys-v0)..."
(cd "$XTASK_DIR" && cargo run -p xtask -- wasm-bindgen imgui-sys-v0)

# ---------- Step 2: Build cimgui provider ----------
PROVIDER_OUT="$XTASK_DIR/target/web-demo"
if [[ ! -f "$PROVIDER_OUT/imgui-sys-v0.wasm" ]]; then
    echo "==> [2/6] Building cimgui provider (Emscripten)..."
    (cd "$XTASK_DIR" && cargo run -p xtask -- build-cimgui-provider)
else
    echo "==> [2/6] Provider already built, skipping. (Delete $PROVIDER_OUT to rebuild)"
fi

# ---------- Step 3: Build our WASM crate ----------
echo "==> [3/6] Building WASM crate (wasm-pack)..."
wasm-pack build "$ROOT/wasm" --target web $PROFILE

# ---------- Step 4: Patch wasm binary to import shared memory ----------
echo "==> [4/6] Patching $WASM_BIN (import env.memory)..."
TMP_WAT="$PKG/__tmp.wat"
TMP_PATCHED="$PKG/__tmp_patched.wat"

wasm-tools print "$WASM_BIN" -o "$TMP_WAT"

python3 -c "
import sys
with open('$TMP_WAT') as f:
    lines = f.readlines()

result = []
for line in lines:
    s = line.strip()
    # Skip internal memory definition (not memory.* instructions)
    if s.startswith('(memory') and not s.startswith('(memory.'):
        continue
    # Skip existing memory export
    if s == '(export \"memory\" (memory 0))':
        continue
    result.append(line)
    # Insert import + export right after (module ...)
    if s.startswith('(module'):
        result.append('  (import \"env\" \"memory\" (memory (;0;) $INIT_PAGES $MAX_PAGES))\n')
        result.append('  (export \"memory\" (memory 0))\n')

with open('$TMP_PATCHED', 'w') as f:
    f.writelines(result)
"

wasm-tools parse "$TMP_PATCHED" -o "$WASM_BIN"
rm -f "$TMP_WAT" "$TMP_PATCHED"
echo "    ✓ wasm_bg.wasm imports memory from env (init=$INIT_PAGES, max=$MAX_PAGES)"

# ---------- Step 5: Patch JS glue to inject shared memory ----------
echo "==> [5/6] Patching $JS_GLUE (inject env.memory)..."

python3 -c "
import sys

with open('$JS_GLUE') as f:
    code = f.read()

if '__imgui_shared_memory' in code:
    print('    ✓ wasm.js already patched')
    sys.exit(0)

# Find __wbg_get_imports() and insert env.memory before the return statement
marker = 'function __wbg_get_imports()'
if marker not in code:
    print('✗ Could not find', marker, file=sys.stderr)
    sys.exit(1)

fn_start = code.index(marker)
return_idx = code.index('    return {', fn_start)

inject = '''
    // Shared memory for imgui-sys-v0 provider + main module
    const __shared_mem = globalThis.__imgui_shared_memory;
    if (!__shared_mem) throw new Error('globalThis.__imgui_shared_memory not set — check index.html');

'''
code = code[:return_idx] + inject + code[return_idx:]

# Add env.memory to the return object (before closing brace)
return_idx2 = code.index('    return {', code.index(marker))
brace = 0
i = return_idx2
while i < len(code):
    if code[i] == '{': brace += 1
    elif code[i] == '}':
        brace -= 1
        if brace == 0:
            code = code[:i] + '        \"env\": { memory: __shared_mem },\n    ' + code[i:]
            break
    i += 1

with open('$JS_GLUE', 'w') as f:
    f.write(code)
print('    ✓ wasm.js injects shared memory as env.memory')
"

# ---------- Step 6: Copy provider files to pkg/ ----------
echo "==> [6/6] Copying provider files to pkg/..."
cp "$PROVIDER_OUT/imgui-sys-v0.js"         "$PKG/"
cp "$PROVIDER_OUT/imgui-sys-v0.wasm"       "$PKG/"
cp "$PROVIDER_OUT/imgui-sys-v0-wrapper.js" "$PKG/"
echo "    ✓ Provider files copied"

echo ""
echo "Done. Serve with:  python3 -m http.server -d wasm 8080"
