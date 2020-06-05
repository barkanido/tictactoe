use crate::board::Board;
use crate::board::Move;
use crate::board::Player;

pub fn minimax(board: &Board, depth: usize, player: Player) -> (Option<(usize, usize)>, isize) {
    let mut best_move = None;
    let mut score = match player {
        Player::Computer => isize::MIN,
        Player::Human => isize::MAX,
    };
    let winner = board.get_winner();
    if depth == 0 || winner.is_some() {
        let score = match winner {
            Some(player) => match player {
                Player::Computer => 1,
                Player::Human => -1,
            },
            None => 0,
        };
        return (best_move, score);
    }

    for spot in board.iter_free_spots() {
        let (row, col) = spot;
        let mut cloned_board = board.clone();
        cloned_board
            .play_move(&Move::new(row, col, player))
            .unwrap();
        let (_, current_score) = minimax(&cloned_board, depth - 1, player.get_opponent());
        match player {
            Player::Computer => {
                if score < current_score {
                    score = current_score;
                    best_move = Some((row, col));
                }
            }
            Player::Human => {
                if current_score < score {
                    score = current_score;
                    best_move = Some((row, col));
                }
            }
        }
    }
    (best_move, score)
}

#[cfg(test)]
mod minimax_tests {

    use super::*;
    #[test]
    fn minimax_test() {
        let mut board = Board::new(3, 3, 3);
        //let m = board::Move::new(0, 2, board::Spot::Human);
        board.play_move(&Move::new(0, 2, Player::Computer));
        board.play_move(&Move::new(1, 0, Player::Computer));
        board.play_move(&Move::new(2, 0, Player::Computer));
        board.play_move(&Move::new(0, 0, Player::Human));
        board.play_move(&Move::new(2, 1, Player::Human));
        board.play_move(&Move::new(2, 2, Player::Human));
        // println!("{:?}", minimax(&mut board, board.size(), Player::Computer));
    }

    #[test]
    fn minimax_test2() {
        let mut board = Board::new(3, 3, 3);
        //let m = board::Move::new(0, 2, board::Spot::Human);
        board.play_move(&Move::new(0, 2, Player::Computer));
        //board.play_move(&Move::new(0, 2, Spot::Computer));
        //board.play_move(&Move::new(2, 0, Spot::Computer));
        board.play_move(&Move::new(0, 1, Player::Human));
        board.play_move(&Move::new(0, 2, Player::Human));
        //board.play_move(&Move::new(2, 2, Spot::Human));
        // println!("{:?}", minimax(&mut board, board.size(), Player::Computer));
    }
}
