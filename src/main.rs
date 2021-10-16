use enwin::*;
use winit::event::{Event, VirtualKeyCode, ElementState, KeyboardInput, WindowEvent};
use winit::event_loop::{EventLoop, ControlFlow};

fn main() {
    let mut game = Game::new();

    let mut event_loop = winit::event_loop::EventLoop::new();

    let window = Game::init_window("Hello window", 512, 512, &event_loop);


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