use encore::*;
use winit::event::{Event, VirtualKeyCode, ElementState, KeyboardInput, WindowEvent};
use winit::event_loop::{EventLoop, ControlFlow};

pub struct Game {
    game : Logic
}

impl Game {

    pub fn new() -> Game {
        Self {
            game: Logic::new()
        }
    }

    pub fn init_window(
        name : &str,
        width : u32,
        height : u32,
        event_loop: &EventLoop<()>) -> winit::window::Window {
        winit::window::WindowBuilder::new()
            .with_title(name)
            .with_inner_size(winit::dpi::LogicalSize::new(width, height))
            .build(&event_loop)
            .expect("Failed to create window")
    }
    
}
