extern crate glfw;
use std::sync::mpsc::Receiver;
use encore::*;

use glfw::{Action, Context, Glfw, Key};

pub struct Window {
    win : glfw::Window,
    events : Receiver<(f64, glfw::WindowEvent)>
}

pub struct Game {
    window : Window,
    glfw : Glfw
}

impl Window {
    pub fn new(glfw : &glfw::Glfw) -> Window {
        let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");
        Self {
            win : window,
            events
        }
    }

    pub fn should_close(&self) -> bool {
        self.win.should_close()
    }


}

impl Game {
    pub fn new() -> Game {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        Self {
            window : Window::new(&glfw),
            glfw
        }
    }

    pub fn game_loop(&mut self) {
        while !self.window.should_close() {
            self.glfw.poll_events();

        }
    }
}