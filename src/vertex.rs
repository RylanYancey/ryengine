
/// A Vertex Position
#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}

impl Vertex {
    pub fn new (pos: [f32; 2]) -> Self {
        Self { position: pos }
    }
}

implement_vertex!(Vertex, position);

/// The Translation of the Entity
/// Multiplied by the Position of Each Vertex on the GPU. 
#[derive(Copy, Clone)]
pub struct Translation {
    pub translation: [f32; 2]
}

impl Translation {
    pub fn new (x: f32, y: f32) -> Self {
        Self {
            translation: [x, y]
        }
    }
}
