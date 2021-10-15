use enwin::*;

fn main() {
    let mut game = Game::new();

    let mut win = Window::init_window("Hello window", 512, 512);

    Window::main_loop(win.event_loop);

}