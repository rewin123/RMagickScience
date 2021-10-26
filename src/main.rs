use enwin::*;
use async_std::sync::Arc;

fn main() {
    let mut game = Game::new();
    game.game_loop();

    let sysres = encore::ResourceManager::new();
}