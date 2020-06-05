use rand::seq::IteratorRandom;
use rand::thread_rng;
use std::error::Error;
use std::fmt;
use std::iter;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    Human,
    Computer,
}

impl Player {
    pub fn get_opponent(&self) -> Player {
        match self {
            Player::Human => Player::Computer,
            Player::Computer => Player::Human,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::Computer => write!(f, "Computer"),
            Player::Human => write!(f, "You"),
        }
    }
}

#[derive(Clone)]
pub struct Board {
    human_symbol: String,
    computer_symbol: String,
    pub grid: Grid,
    win_length: usize,
}
#[derive(Debug)]
struct MoveError {
    details: String,
}

impl MoveError {
    fn new(msg: &str) -> MoveError {
        MoveError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for MoveError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Copy, Clone)]
pub struct Move {
    row: usize,
    col: usize,
    player: Player,
}

impl Move {
    pub fn new(row: usize, col: usize, player: Player) -> Move {
        Move { col, row, player }
    }
}

#[derive(Clone)]
pub struct Grid {
    pub rows: usize,
    pub cols: usize,
    v: Vec<Option<Player>>,
}

impl Grid {
    fn new(rows: usize, cols: usize) -> Grid {
        let size = rows * cols;
        let mut v: Vec<Option<Player>> = Vec::with_capacity(size);
        for _ in 0..size {
            v.push(None);
        }
        Grid { rows, cols, v }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Option<Player>> {
        self.v.iter()
    }

    pub fn iter_free_spots<'a>(&'a self) -> impl Iterator<Item = (usize, usize)> + 'a {
        self.iter()
            .enumerate()
            .filter(|(_idx, op)| op.is_none())
            .map(move |(idx, _)| self.get_tuple_index(idx))
    }

    pub fn rows_iter(&self) -> impl Iterator<Item = impl Iterator<Item = &Option<Player>>> {
        (0..self.rows).map(move |row_index| self.row_iter(row_index))
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.rows + column
    }

    fn get_tuple_index(&self, index: usize) -> (usize, usize) {
        (index / self.rows, index % self.rows)
    }

    pub fn row_iter(&self, row_index: usize) -> impl Iterator<Item = &Option<Player>> {
        let start = self.get_index(row_index, 0);
        let end = start + self.cols;
        self.v[start..end].iter()
    }

    fn idx_in_range(&self, row: isize, col: isize) -> bool {
        0 <= row && row < self.rows as isize && 0 <= col && col < self.cols as isize
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = Option<Player>;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        let (row, col) = idx;
        &self.v[self.get_index(row, col)]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        let (row, col) = idx;
        let index = self.get_index(row, col);
        &mut self.v[index]
    }
}

impl Board {
    pub fn new(rows: usize, cols: usize, win_length: usize) -> Board {
        Board {
            human_symbol: "O".to_string(),
            computer_symbol: "X".to_string(),
            grid: Grid::new(rows, cols),
            win_length,
        }
    }

    pub fn play_move(&mut self, cur_move: &Move) -> Result<(), Box<dyn Error>> {
        if !self
            .grid
            .idx_in_range(cur_move.row as isize, cur_move.col as isize)
        {
            return Err(Box::new(MoveError::new("index out of bounds")));
        }
        self.grid[(cur_move.row, cur_move.col)] = Some(cur_move.player);
        Ok(())
    }

    // pub fn unplay_move(&mut self, spot: (usize, usize)) {
    //     self.grid[(spot.0, spot.1)] = None;
    // }

    pub fn play_random_move(&mut self, player: Player) {
        let mut rng = thread_rng();
        let (row, col) = self.grid.iter_free_spots().choose(&mut rng).unwrap();
        self.play_move(&Move::new(row, col, player)).unwrap();
    }

    pub fn count_free_spots(&self) -> usize {
        self.grid.iter_free_spots().count()
    }

    // pub fn iter(&self) -> impl Iterator<Item = &Option<Player>> {
    //     self.grid.iter()
    // }

    pub fn iter_free_spots<'a>(&'a self) -> impl Iterator<Item = (usize, usize)> + 'a {
        self.grid.iter_free_spots()
    }

    pub fn is_game_over(&self) -> bool {
        self.grid.iter().all(|s| s.is_some())
    }

    pub fn is_empty(&self) -> bool {
        self.grid.iter().all(|s| s.is_none())
    }

    fn is_last_move_won_back_diagonal(&self, cur_move: &Move) -> bool {
        let mut count = 1;
        let mut i = cur_move.row as isize + 1;
        let mut j = cur_move.col as isize + 1;
        while self.grid.idx_in_range(i, j) {
            if self.is_player_in_spot(i, j, cur_move.player) {
                count += 1;
            } else {
                break;
            }
            if count == self.win_length {
                return true;
            }
            i += 1;
            j += 1;
        }
        i = cur_move.row as isize - 1;
        j = cur_move.col as isize - 1;
        while self.grid.idx_in_range(i, j) {
            if self.is_player_in_spot(i, j, cur_move.player) {
                count += 1;
            } else {
                break;
            }
            if count == self.win_length {
                return true;
            }
            i -= 1;
            j -= 1;
        }
        false
    }

    fn is_last_move_won_forward_diagonal(&self, cur_move: &Move) -> bool {
        let mut count = 1;
        let mut i = cur_move.row as isize - 1;
        let mut j = cur_move.col as isize + 1;
        while self.grid.idx_in_range(i, j) {
            if self.is_player_in_spot(i, j, cur_move.player) {
                count += 1;
            } else {
                break;
            }
            if count == self.win_length {
                return true;
            }
            i -= 1;
            j += 1;
        }
        i = cur_move.row as isize + 1;
        j = cur_move.col as isize - 1;
        while self.grid.idx_in_range(i, j) {
            if self.is_player_in_spot(i, j, cur_move.player) {
                count += 1;
            } else {
                break;
            }
            if count == self.win_length {
                return true;
            }
            i += 1;
            j -= 1;
        }
        false
    }

    fn is_last_move_won_row(&self, cur_move: &Move) -> bool {
        let mut count = 1;
        let mut i = cur_move.row as isize;
        let mut j = cur_move.col as isize + 1;
        while self.grid.idx_in_range(i, j) {
            if self.is_player_in_spot(i, j, cur_move.player) {
                count += 1;
            } else {
                break;
            }
            if count == self.win_length {
                return true;
            }
            j += 1;
        }
        i = cur_move.row as isize;
        j = cur_move.col as isize - 1;
        while self.grid.idx_in_range(i, j) {
            if self.is_player_in_spot(i, j, cur_move.player) {
                count += 1;
            } else {
                break;
            }
            if count == self.win_length {
                return true;
            }
            j -= 1;
        }
        false
    }

    fn is_last_move_won_col(&self, cur_move: &Move) -> bool {
        let mut count = 1;
        let mut i = cur_move.row as isize + 1;
        let mut j = cur_move.col as isize;
        while self.grid.idx_in_range(i, j) {
            if self.is_player_in_spot(i, j, cur_move.player) {
                count += 1;
            } else {
                break;
            }
            if count == self.win_length {
                return true;
            }
            i += 1;
        }
        i = cur_move.row as isize - 1;
        j = cur_move.col as isize;
        while self.grid.idx_in_range(i, j) {
            if self.is_player_in_spot(i, j, cur_move.player) {
                count += 1;
            } else {
                break;
            }
            if count == self.win_length {
                return true;
            }
            i -= 1;
        }
        false
    }

    fn is_player_in_spot(&self, i: isize, j: isize, player: Player) -> bool {
        match self.grid[(i as usize, j as usize)] {
            None => false,
            Some(p) => player == p,
        }
    }

    pub fn is_wining_move(&self, cur_move: &Move) -> bool {
        self.is_last_move_won_back_diagonal(cur_move)
            || self.is_last_move_won_forward_diagonal(cur_move)
            || self.is_last_move_won_col(cur_move)
            || self.is_last_move_won_row(cur_move)
    }

    pub fn get_winner(&self) -> Option<Player> {
        for row in 0..self.grid.rows {
            for col in 0..self.grid.cols {
                if let Some(player) = self.grid[(row, col)] {
                    if self.is_wining_move(&Move::new(row, col, player)) {
                        return Some(player);
                    }
                }
            }
        }
        None
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let line_sep = iter::repeat("-")
            .take((2 * self.grid.cols) + 1)
            .collect::<String>();
        writeln!(f, "\n{}", line_sep)?;
        for row in self.grid.rows_iter() {
            write!(f, "|")?;
            for player in row {
                match player {
                    None => write!(f, "{}", " ")?,
                    Some(Player::Human) => write!(f, "O")?,
                    Some(Player::Computer) => write!(f, "X")?,
                };
                write!(f, "|")?;
            }
            writeln!(f, "\n{}", line_sep)?;
        }
        Ok(())
    }
}
