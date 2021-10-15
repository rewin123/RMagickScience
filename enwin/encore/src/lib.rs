use legion::*;


pub struct Logic {
    world : World
}

impl Logic {
    pub fn new() -> Logic {
        Self {
            world : World::default()
        }
    }

    pub fn Step(&self, dt : f32) {

    }
}