use encore::*;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowId},
};


pub struct Game {
    logick : Logic,
    window : Window,
    event_loop : EventLoop<()>,
    render : enrender::Render
}


impl Game {
    pub fn new() -> Game {

        let event_loop = EventLoop::new();
        let window = Window::new(&event_loop).unwrap();
        let render = enrender::Render::new(&window);

        Self {
            event_loop,
            window,
            render,
            logick : Logic::new()
        }   

        
    }

    pub fn game_loop(self) {
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::WindowEvent {
                    event: WindowEvent::Resized(size),
                    ..
                } => {
                    
                }
                Event::RedrawRequested(_) => {
                    
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,
                _ => {}
            }
        });
    }
}
