use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

/// Adds transform fields and helper methods to a struct.
///
/// Injected fields:
/// - `position: glam::Vec3`
/// - `rotation: glam::Quat`
/// - `scale: glam::Vec3`
/// - `local_front: glam::Vec3`
/// - `local_right: glam::Vec3`
/// - `local_up: glam::Vec3`
#[proc_macro_attribute]
pub fn with_transform(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;
    let vis = &input.vis;
    let attrs = &input.attrs;

    // Preserve existing named fields; for tuple structs return original.
    let mut existing_names = Vec::new();
    let mut existing_fields = Vec::new();
    match &input.fields {
        syn::Fields::Named(named) => {
            for f in named.named.iter() {
                if let Some(ident) = &f.ident {
                    existing_names.push(ident.to_string());
                }
                existing_fields.push(quote! { #f });
            }
        }
        syn::Fields::Unit => {}
        syn::Fields::Unnamed(_) => return TokenStream::from(quote! { #input }),
    }

    let want = vec![
        ("position", quote! { pub position: glam::Vec3 }),
        ("rotation", quote! { pub rotation: glam::Quat }),
        ("scale", quote! { pub scale: glam::Vec3 }),
        ("local_front", quote! { pub local_front: glam::Vec3 }),
        ("local_right", quote! { pub local_right: glam::Vec3 }),
        ("local_up", quote! { pub local_up: glam::Vec3 }),
    ];

    let mut all_field_tokens = existing_fields.clone();
    for (name_s, tokens) in want.into_iter() {
        if !existing_names.iter().any(|n| n == name_s) {
            all_field_tokens.push(tokens);
        }
    }

    let expanded = quote! {
        #(#attrs)*
        #vis struct #name {
            #(#all_field_tokens),*
        }

        impl #name {
            pub fn update_local_axes(&mut self) {
                // Convert rotation to a 3x3 matrix, transpose it, and use its rows
                // as the local axes. This mirrors the GLSL/GLM approach.
                let rot_mat: glam::Mat3 = glam::Mat3::from_quat(self.rotation);
                let t = rot_mat.transpose();
                self.local_front = -t.row(2);
                self.local_right = t.row(0);
                self.local_up = t.row(1);
            }

            pub fn set_position(&mut self, p: glam::Vec3) { self.position = p; }
            pub fn set_rotation(&mut self, r: glam::Quat) { self.rotation = r; self.update_local_axes(); }
            pub fn set_scale(&mut self, s: glam::Vec3) { self.scale = s; }

            pub fn get_position_matrix(&self) -> glam::Mat4 {
                glam::Mat4::from_translation(-self.position)
            }
            pub fn move_forward(&mut self, speed: f32, delta: f32) {
                self.position += self.local_front * speed  * delta;
            }
            pub fn move_backward(&mut self, speed: f32, delta: f32) {
                self.position -= self.local_front * speed  * delta;
            }
            pub fn move_left(&mut self, speed: f32, delta: f32) {
                self.position -= self.local_right * speed  * delta;
            }
            pub fn move_right(&mut self, speed: f32, delta: f32) {
                self.position += self.local_right * speed  * delta;
            }
            pub fn move_up(&mut self, speed: f32, delta: f32) {
                self.position += self.local_up * speed  * delta;
            }
            pub fn move_down(&mut self, speed: f32, delta: f32) {
                self.position -= self.local_up * speed  * delta;
            }
            pub fn move_global_up(&mut self, speed: f32, delta: f32) {
                self.position += Vec3::new(0.0, 1.0, 0.0) * speed  * delta;
            }
            pub fn move_global_down(&mut self, speed: f32, delta: f32) {
                self.position -= Vec3::new(0.0, 1.0, 0.0) * speed  * delta;
            }
        }
    };

    TokenStream::from(expanded)
}
