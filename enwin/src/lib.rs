use std::sync::mpsc::Receiver;
use encore::*;

use ash::vk;
use std::ffi::CString;
use std::ptr;

use glfw::{Action, Context, Glfw, Key};
use ash;

use ash::extensions::ext::DebugUtils;
use ash::extensions::khr::Surface;

#[cfg(target_os = "windows")]
use ash::extensions::khr::Win32Surface;
pub struct Window {

}

pub struct Game {
    logick : Logic,
    window : Window,
}

impl Window {
    pub fn new() -> Window {

        Self {
        }
    }


}

impl Game {
    pub fn new() -> Game {
        let mut win = Window::new();
        Self {
            window : win,
            logick : Logic::new(),
        }
    }

    pub fn game_loop(&mut self) {

    }
}

#[cfg(all(windows))]
pub fn required_extension_names() -> Vec<*const i8> {
    vec![
        Surface::name().as_ptr(),
        Win32Surface::name().as_ptr(),
        DebugUtils::name().as_ptr(),
    ]
}
