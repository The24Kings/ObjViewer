pub mod material;
pub mod mesh;

use crate::game::Material;
use crate::game::Mesh;

pub trait Renderable {
    fn material(&self) -> &Material;
    fn mesh(&self) -> &Mesh;
}
