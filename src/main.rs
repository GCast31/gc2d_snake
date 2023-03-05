mod game;
mod snake;
mod map;

use game::Game;
use gc2d::gc2d::Gc2d;

fn main() {
    let gc2d = Gc2d::new().run(Game::new());
}
