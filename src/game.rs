

use std::fmt;
use std::fmt::{Display, Formatter};
use std::process;

use utils;

const BOARD_SIZE: usize = 3;

pub struct Game {
    pub board: Vec<Cell>,
    pub player: Player,
}

impl Game {
    pub fn new() -> Game {
        let mut board = Vec::<Cell>::with_capacity(BOARD_SIZE);
        for i in 1...(BOARD_SIZE.pow(2)) {
            board.push(Cell::Empty(i))
        }
        Game {
            board: board,
            player: Player::X,
        }
    }

    pub fn update(&self, input: usize) -> Game {

        let mut game = match update_board(self, input) {
            Ok(game) => game,
            Err(game) => {
                println!("You must play a cell that hasn't already been played. \
                          Please try again.");
                let new_input = utils::get_input();
                game.update(new_input)
            }
        };

        // Check to see if anyone won
        match determine_winner(&game.board) {
            None => {
                game.player = match game.player {
                    Player::X => Player::O,
                    Player::O => Player::X,
                    Player::Cats => Player::Cats,
                };
                game
            }
            Some(Player::X) => {
                game.display();
                println!("X won!");
                process::exit(0);
            }
            Some(Player::O) => {
                game.display();
                println!("O won!");
                process::exit(0);
            }
            Some(Player::Cats) => {
                game.display();
                println!("Cats!");
                process::exit(0);
            }

        }
    }

    pub fn display(&self) {
        for (i, row) in self.board.chunks(BOARD_SIZE).enumerate() {
            for (j, cell) in row.iter().enumerate() {
                print!("{}", cell)
            }
            println!("")
        }
        println!("")
    }
}

fn determine_winner(board: &Vec<Cell>) -> Option<Player> {
    let winning_strategies = vec![vec![0, 1, 2],
                                  vec![3, 4, 5],
                                  vec![6, 7, 8],
                                  vec![0, 3, 6],
                                  vec![1, 4, 7],
                                  vec![2, 5, 8],
                                  vec![0, 4, 8],
                                  vec![2, 4, 6]];
    let mut p = 0;
    let mut x = vec![];
    let mut o = vec![];
    for (index, player) in board.iter().enumerate() {
        if *player == Cell::Marked(Player::X) {
            x.push(index)
        };
        if *player == Cell::Marked(Player::O) {
            o.push(index)
        }
    }
    if winning_strategies.contains(&x) {
        return Some(Player::X);
    };
    if winning_strategies.contains(&o) {
        return Some(Player::O);
    };
    for player in board.iter() {
        if *player == Cell::Marked(Player::X) {
            p += 1
        };
        if *player == Cell::Marked(Player::O) {
            p += 1
        }
    }
    if p == BOARD_SIZE.pow(2 as u32) {
        return Some(Player::Cats);
    }
    None
}

fn update_board(game: &Game, input: usize) -> Result<Game, Game> {
    let mut game = game.clone();
    match game.board[input - 1] {
        Cell::Empty(n) => {
            game.board[input - 1] = Cell::Marked(game.player);
            Ok(game)
        }
        _ => Err(game),
    }
}

impl Clone for Game {
    fn clone(&self) -> Self {
        Game {
            board: self.board.to_vec(),
            player: self.player.clone(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Player {
    X,
    O,
    Cats,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Cell {
    Marked(Player),
    Empty(usize),
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            &Cell::Marked(Player::O) => write!(f, " O "),
            &Cell::Marked(Player::X) => write!(f, " X "),
            &Cell::Empty(n) => write!(f, "[{}]", n),
            &Cell::Marked(Player::Cats) => write!(f, "[ ]"),
        }
    }
}
