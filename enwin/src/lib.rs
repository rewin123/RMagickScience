use encore::*;
use winit::event::{Event, VirtualKeyCode, ElementState, KeyboardInput, WindowEvent};
use winit::event_loop::{EventLoop, ControlFlow};

pub struct Window {
    win : winit::window::Window,
    pub event_loop : EventLoop<()>
}

pub struct Game {
    game : Logic,
    windows : Vec<Window>
}

impl Window {
    pub fn init_window(
        name : &'static str,
        width : u32,
        height : u32) -> Window {
        let mut event_loop = EventLoop::new();
        let mut win = winit::window::WindowBuilder::new()
            .with_title(name)
            .with_inner_size(winit::dpi::LogicalSize::new(width, height))
            .build(&event_loop)
            .expect("Failed to create window");
        Self {
            win,
            event_loop
        }
    }

    pub fn main_loop(event_loop: EventLoop<()>) {
        event_loop.run(move |event, _, control_flow| {

            match event {
                | Event::WindowEvent { event, .. } => {
                    match event {
                        | WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit
                        },
                        | WindowEvent::KeyboardInput { input, .. } => {
                            match input {
                                | KeyboardInput { virtual_keycode, state, .. } => {
                                    match (virtual_keycode, state) {
                                        | (Some(VirtualKeyCode::Escape), ElementState::Pressed) => {
                                            dbg!();
                                            *control_flow = ControlFlow::Exit
                                        },
                                        | _ => {},
                                    }
                                },
                            }
                        },
                        | _ => {},
                    }
                },
                _ => (),
            }

        })
    }
}

impl Game {

    pub fn new() -> Game {
        Self {
            game: Logic::new(),
            windows: Vec::new()
        }
    }
}
