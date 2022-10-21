
pub struct Camera {
    pub perspective:[f32; 2]
}

impl Camera {
    pub fn new (x: f32, y: f32) -> Self {
        Self {
            perspective: [-x, -y]
        }
    }

    pub fn move_horizontal (&mut self, amt: f32) {
        self.perspective[0] -= amt;
    }

    pub fn move_vertical (&mut self, amt: f32) {
        self.perspective[1] -= amt;
    }
}