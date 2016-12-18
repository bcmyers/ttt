#![allow(dead_code, unused_variables)]
#![feature(inclusive_range_syntax, slice_patterns)]

mod game;
mod utils;

use game::Game;

fn main() {
    let mut game = Game::new();
    loop {
        let input = utils::get_input();
        game = game.update(input);
        game.display();
    }
}
