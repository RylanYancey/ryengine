
#[macro_use]
extern crate glium;

pub mod app;
pub mod vertex;
pub mod input;
pub mod entity;
pub mod components;

use app::App;

fn main() {
    App::new()  
        .add_startup_system(start)
        .add_system(update)
        .run();
}

pub fn start () {
    
}

pub fn update () {
    
}

pub fn draw () {

}
