#![allow(dead_code)]

mod board;

use crate::board::*;
use rand::seq::SliceRandom;
use std::cmp::{max, min};
use std::collections::HashMap;

// struct Game {
//     current_turn: Turn, //btw, current turn can be determined from game state alone
//     history: Vec<Board>,
// }

struct TicTacToeAI {
    turn: Turn,
    state_evals: HashMap<Board, i8>,
}

impl TicTacToeAI {
    fn from_turn(turn: Turn) -> Self {
        Self {
            turn: turn,
            state_evals: HashMap::new(),
        }
    }
    // * The AI have been altered so that the only maximize if the board is playing their turn; otherwise, they choose the minimizing move, as though they were playing for their opponent.
    fn choose_move(&mut self, board: Board) -> Board {
        self.eval(board, board.turn());
        let next = board.succ(board.turn());

        let mut instant_wins = Vec::new();
        let mut wins = Vec::new();
        let mut draws = Vec::new();
        let mut losses = Vec::new();

        for n in next.into_iter() {
            if n.evaluate(self.turn) == 1 {
                instant_wins.push(n);
            } else if let Some(e) = self.state_evals.get(&n) {
                match e {
                    1 => wins.push(n),
                    0 => draws.push(n),
                    -1 => losses.push(n),
                    _ => unreachable!(),
                }
            } else {
                panic!("State not found in evals: \n{}", n);
            }
        }

        // ? Delete these debug prints later.
        // instant_wins.iter().for_each(|b| println!("instant win: \n{}", b));
        // wins.iter().for_each(|b| println!("win: \n{}", b));
        // draws.iter().for_each(|b| println!("draw: \n{}", b));
        // losses.iter().for_each(|b| println!("loss: \n{}", b));

        // TODO: Not only choose from the winning moves, but choose the winning move which leads to the most winning outcomes.
        if !instant_wins.is_empty() {
            *instant_wins.choose(&mut rand::thread_rng()).unwrap()
        } else if !wins.is_empty() {
            *wins.choose(&mut rand::thread_rng()).unwrap()
        } else if !draws.is_empty() {
            *draws.choose(&mut rand::thread_rng()).unwrap()
        } else {
            *losses.choose(&mut rand::thread_rng()).unwrap()
        }
    }
    fn eval(&mut self, board: Board, t: Turn) -> i8 {
        if self.turn == t {
            self.minimax_evaluate(board, 100, true)
        } else if self.turn == t.other() {
            self.minimax_evaluate(board, 100, false)
        } else {
            unreachable!();
        }
    }
    fn minimax_evaluate(&mut self, board: Board, depth: u8, maximizing: bool) -> i8 {
        if let Some(&eval) = self.state_evals.get(&board) {
            return eval;
        }

        if depth == 0 || board.accepts() {
            let eval = board.evaluate(self.turn);
            self.state_evals.insert(board, eval);
            return eval;
        }

        let mut eval: i8;
        if maximizing {
            let mut maxeval = i8::MIN;
            for next in board.succ(self.turn) {
                eval = self.minimax_evaluate(next, depth - 1, false);
                maxeval = max(maxeval, eval);
            }
            self.state_evals.insert(board, maxeval);
            maxeval
        } else {
            let mut mineval = i8::MAX;
            for next in board.succ(self.turn.other()) {
                eval = self.minimax_evaluate(next, depth - 1, true);
                mineval = min(mineval, eval);
            }
            self.state_evals.insert(board, mineval);
            mineval
        }
    }
}

impl Default for TicTacToeAI {
    fn default() -> Self {
        Self {
            turn: Turn::X,
            state_evals: HashMap::new(),
        }
    }
}

// TODO: Implement actual game and game loop.

// Prints a board with a fancy marking telling you who moved.
fn print_move(board: Board, who_moved: String) {
    println!("\n{} moved: \n========\n{}========\n", who_moved, board)
}

// Reads a command from stdin.
fn receive_move(board: Board, turn: Turn) -> Option<Board> {
    println!("Your move.");
    let mut input = String::new();
    while let Ok(_) = std::io::stdin().read_line(&mut input) {
        if let Ok(key) = input.trim().parse::<i8>() {
            match key {
                1..=9 => {
                    match Board::try_move(&board, key, turn) {
                        None => {
                            println!("Invalid move... try again, but choose an empty square.");
                        }
                        some_board => return some_board,
                    }
                }
                0 => return None,
                _ => {
                    println!("Invalid number given... try 1-9! Or 0 to quit.");
                }
            }
        } else {
            println!("Invalid command.  Are you even giving a number?  Try again.");
        }
        input.clear();
    }
    None
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut p1 = TicTacToeAI::from_turn(Turn::X);
    let mut p2 = TicTacToeAI::from_turn(Turn::O);
    // let s: Board = Board::from_state([
    //     [Square::X, Square::Empty, Square::Empty],
    //     [Square::Empty, Square::Empty, Square::O],
    //     [Square::Empty, Square::Empty, Square::Empty],
    // ]);
    // println!("{}", s);
    // let eval1 = p1.eval(s, Turn::X);
    // let eval2 = p2.eval(s, Turn::X);
    // println!("{}, {}", eval1, eval2);
    // let a = p1.choose_move(s);
    // println!("{}", a);

    let mut board: Board = Board::new();
    println!("Guide:\n");
    board.display()?;
    println!("\nOffer a number from 1-9 accordingly to place your square.  Offer 0 to end the game.");
    while !board.accepts() {
        if let Some(b) = receive_move(board, Turn::X) {
            board = b;
            print_move(board, "Human (X)".into());
            std::thread::sleep(std::time::Duration::from_secs(1));
            // board.display()?;

            if board.accepts() {
                break;
            }

            board = p2.choose_move(b);
            print_move(board, "AI (O)".into());
            std::thread::sleep(std::time::Duration::from_secs(1));

            // board.display()?;
        } else {
            break;
        }
    }
    println!("{}", board.outcome());

    Ok(())
}
