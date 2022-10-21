
use crate::components::Component;

pub struct Entity {
    pub components: Vec<Box<dyn Component>>,
    pub transform: [f32; 3]
}

impl Entity {
    pub fn new (components: Vec<Box<dyn Component>>, transform: [f32; 3]) -> Self {
        let mut me = Self {
            components, transform
        };

        

        me
    }
}