
use crate::entity::Entity;

pub trait Component {
    fn start (&mut self, parent: &mut Entity);
    fn update (&mut self, parent: &mut Entity);
}

mod draw;
mod camera;

pub use draw::Draw;
pub use camera::Camera;