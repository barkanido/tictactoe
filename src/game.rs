use crate::board::{Board, Move, Player};
use crate::minimax::minimax;
use std::error::Error;
use std::num::ParseIntError;
use std::time::Instant;
use std::{fmt, io, thread, time};

pub fn play_game(mut board: Board) {
    let mut winner: Option<Player> = None;
    let first_player = Player::Human;
    let mut current_player = first_player;
    while winner.is_none() {
        println!("{}", board);
        // if there is a winner, announce and exit
        let player = board.get_winner();
        if player.is_some() {
            winner = player;
            println!("{} wins!", winner.unwrap());
            break;
        } else if board.is_game_over() {
            println!("a tie!");
            break;
        }
        match current_player {
            Player::Human => human_turn(&mut board),
            Player::Computer => computer_turn(&mut board),
        };
        current_player = current_player.get_opponent();
    }
}

fn human_turn(board: &mut Board) {
    // read next move
    let mut required_move = String::new();
    loop {
        println!("Enter comma separated move: (\"row,column\"): ");
        io::stdin()
            .read_line(&mut required_move)
            .expect("Failed to read line");
        match parse_move(&required_move) {
            Ok(cur_move) => {
                let (row, col) = cur_move;
                let cur_move = Move::new(row, col, Player::Human);
                // perform move
                if board.play_move(&cur_move).is_ok() {
                    break;
                }
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}

fn computer_turn(board: &mut Board) {
    thread::sleep(time::Duration::from_secs(1));
    if board.is_game_over() {
        println!("computer: game is over");
    } else if board.is_empty() {
        println!("computer: playing random move");
        board.play_random_move(Player::Computer)
    } else {
        println!("computer: thinking...");
        let now = Instant::now();
        let (suggested_move, _) = minimax(board, board.count_free_spots(), Player::Computer);
        println!("took {:.3} secs", now.elapsed().as_millis() as f64 / 1000.0);
        let (row, col) = suggested_move.unwrap();
        board
            .play_move(&Move::new(row, col, Player::Computer))
            .unwrap();
    }
}

fn parse_move(parsed_args: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let required_move: Result<Vec<_>, ParseIntError> = parsed_args
        .trim()
        .split(',')
        .map(|n| n.parse::<usize>())
        .collect();
    match required_move {
        Ok(m) => {
            if m.len() == 2 {
                Ok((m[0], m[1]))
            } else {
                Err(Box::new(ParseError::new("too many args")))
            }
        }
        Err(e) => Err(Box::new(ParseError::new(format!("{}", e).as_str()))),
    }
}

#[derive(Debug)]
struct ParseError {
    details: String,
}

impl ParseError {
    fn new(msg: &str) -> ParseError {
        ParseError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        &self.details
    }
}
