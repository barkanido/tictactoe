use rand::seq::IteratorRandom;
use rand::thread_rng;
use std::error::Error;
use std::fmt;
use std::iter;
use std::ops::{Index, IndexMut};

const ROWS: usize = 3;
const COLUMNS: usize = 3;
const BOARD_SIZE: usize = ROWS * COLUMNS;

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
    v: [Option<Player>; BOARD_SIZE],
    computer_hash: i32,
    human_hash: i32,
    winnings: [i32; 8],
}

impl Grid {
    fn new() -> Grid {
        let v: [Option<Player>; BOARD_SIZE] = [None; BOARD_SIZE];
        let computer_hash: i32 = 0;
        let human_hash: i32 = 0;
        let winnings: [i32; 8] = [
            0b111,
            0b111000,
            0b111000000,
            0b1001001,
            0b10010010,
            0b100100100,
            0b001010100,
            0b100010001,
        ];

        Grid {
            v,
            computer_hash,
            human_hash,
            winnings,
        }
    }

    fn update_hash(&mut self, player: Player, i: usize, j: usize) {
        let idx = self.get_index(i, j);
        match player {
            Player::Computer => self.computer_hash |= 1 << idx,
            Player::Human => self.human_hash |= 1 << idx,
        }
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
        (0..ROWS).map(move |row_index| self.row_iter(row_index))
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        row * ROWS + column
    }

    fn get_tuple_index(&self, index: usize) -> (usize, usize) {
        (index / ROWS, index % ROWS)
    }

    pub fn row_iter(&self, row_index: usize) -> impl Iterator<Item = &Option<Player>> {
        let start = self.get_index(row_index, 0);
        let end = start + COLUMNS;
        self.v[start..end].iter()
    }

    fn idx_in_range(&self, row: isize, col: isize) -> bool {
        0 <= row && row < ROWS as isize && 0 <= col && col < COLUMNS as isize
    }

    pub fn get_winner_fast(&self) -> Option<Player> {
        let mut computer: i32 = 0;
        let mut human: i32 = 0;
        for row in 0..ROWS {
            for col in 0..COLUMNS {
                let idx = self.get_index(row, col);
                if let Some(player) = self[(row, col)] {
                    match player {
                        Player::Computer => computer |= 1 << idx,
                        Player::Human => human |= 1 << idx,
                    }
                }
            }
        }
        if (&self.winnings).into_iter().any(|&x| x == x & human) {
            return Some(Player::Human);
        } else if (&self.winnings).into_iter().any(|&x| x == x & computer) {
            return Some(Player::Computer);
        }
        None
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
    pub fn new() -> Board {
        Board {
            human_symbol: "O".to_string(),
            computer_symbol: "X".to_string(),
            grid: Grid::new(),
        }
    }

    pub fn play_move(&mut self, cur_move: &Move) -> Result<(), Box<dyn Error>> {
        if !self
            .grid
            .idx_in_range(cur_move.row as isize, cur_move.col as isize)
        {
            return Err(Box::new(MoveError::new("index out of bounds")));
        }
        let (row, col, player) = (cur_move.row, cur_move.col, cur_move.player);
        self.grid[(row, col)] = Some(player);
        self.grid.update_hash(player, row, col);
        Ok(())
    }

    pub fn play_random_move(&mut self, player: Player) {
        let mut rng = thread_rng();
        let (row, col) = self.grid.iter_free_spots().choose(&mut rng).unwrap();
        self.play_move(&Move::new(row, col, player)).unwrap();
    }

    pub fn count_free_spots(&self) -> usize {
        self.grid.iter_free_spots().count()
    }

    pub fn iter_free_spots<'a>(&'a self) -> impl Iterator<Item = (usize, usize)> + 'a {
        self.grid.iter_free_spots()
    }

    pub fn is_game_over(&self) -> bool {
        self.grid.iter().all(|s| s.is_some())
    }

    pub fn is_empty(&self) -> bool {
        self.grid.iter().all(|s| s.is_none())
    }

    pub fn get_winner(&self) -> Option<Player> {
        self.grid.get_winner_fast()
    }
}

const EMPTY_SPOT_SIGN: &str = " ";

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let line_sep = iter::repeat("-")
            .take((2 * COLUMNS) + 1)
            .collect::<String>();
        writeln!(f, "\n{}", line_sep)?;
        for row in self.grid.rows_iter() {
            write!(f, "|")?;
            for player in row {
                match player {
                    None => write!(f, "{}", EMPTY_SPOT_SIGN)?,
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

#[cfg(test)]
mod grid_tests {

    fn play_move(grid: &mut Grid, i: usize, j: usize, player: Player) {
        grid[(i, j)] = Some(player);
        grid.update_hash(player, i, j);
    }

    use super::*;
    #[test]
    fn get_winner_fast_test() {
        // human
        let mut grid = Grid::new();
        assert_eq!(grid.get_winner_fast(), None);
        play_move(&mut grid, 0, 0, Player::Computer);
        play_move(&mut grid, 0, 1, Player::Computer);
        play_move(&mut grid, 0, 2, Player::Computer);
        assert_eq!(grid.get_winner_fast(), Some(Player::Computer));

        let mut grid = Grid::new();
        play_move(&mut grid, 1, 0, Player::Computer);
        play_move(&mut grid, 1, 1, Player::Computer);
        play_move(&mut grid, 1, 2, Player::Computer);
        assert_eq!(grid.get_winner_fast(), Some(Player::Computer));
        let mut grid = Grid::new();
        play_move(&mut grid, 2, 0, Player::Computer);
        play_move(&mut grid, 2, 1, Player::Computer);
        play_move(&mut grid, 2, 2, Player::Computer);
        assert_eq!(grid.get_winner_fast(), Some(Player::Computer));
        let mut grid = Grid::new();
        play_move(&mut grid, 0, 0, Player::Computer);
        play_move(&mut grid, 1, 0, Player::Computer);
        play_move(&mut grid, 2, 0, Player::Computer);
        assert_eq!(grid.get_winner_fast(), Some(Player::Computer));
        let mut grid = Grid::new();
        play_move(&mut grid, 0, 1, Player::Computer);
        play_move(&mut grid, 1, 1, Player::Computer);
        play_move(&mut grid, 2, 1, Player::Computer);
        assert_eq!(grid.get_winner_fast(), Some(Player::Computer));
        let mut grid = Grid::new();
        play_move(&mut grid, 0, 2, Player::Computer);
        play_move(&mut grid, 1, 2, Player::Computer);
        play_move(&mut grid, 2, 2, Player::Computer);
        assert_eq!(grid.get_winner_fast(), Some(Player::Computer));
        let mut grid = Grid::new();
        play_move(&mut grid, 0, 0, Player::Computer);
        play_move(&mut grid, 1, 1, Player::Computer);
        play_move(&mut grid, 2, 2, Player::Computer);
        assert_eq!(grid.get_winner_fast(), Some(Player::Computer));
        let mut grid = Grid::new();
        play_move(&mut grid, 0, 2, Player::Computer);
        play_move(&mut grid, 1, 1, Player::Computer);
        play_move(&mut grid, 2, 0, Player::Computer);
        assert_eq!(grid.get_winner_fast(), Some(Player::Computer));

        // computer
        let mut grid = Grid::new();
        play_move(&mut grid, 0, 0, Player::Human);
        play_move(&mut grid, 0, 1, Player::Human);
        play_move(&mut grid, 0, 2, Player::Human);
        assert_eq!(grid.get_winner_fast(), Some(Player::Human));

        let mut grid = Grid::new();
        play_move(&mut grid, 1, 0, Player::Human);
        play_move(&mut grid, 1, 1, Player::Human);
        play_move(&mut grid, 1, 2, Player::Human);
        assert_eq!(grid.get_winner_fast(), Some(Player::Human));
        let mut grid = Grid::new();
        play_move(&mut grid, 2, 0, Player::Human);
        play_move(&mut grid, 2, 1, Player::Human);
        play_move(&mut grid, 2, 2, Player::Human);
        assert_eq!(grid.get_winner_fast(), Some(Player::Human));
        let mut grid = Grid::new();
        play_move(&mut grid, 0, 0, Player::Human);
        play_move(&mut grid, 1, 0, Player::Human);
        play_move(&mut grid, 2, 0, Player::Human);
        assert_eq!(grid.get_winner_fast(), Some(Player::Human));
        let mut grid = Grid::new();
        play_move(&mut grid, 0, 1, Player::Human);
        play_move(&mut grid, 1, 1, Player::Human);
        play_move(&mut grid, 2, 1, Player::Human);
        assert_eq!(grid.get_winner_fast(), Some(Player::Human));
        let mut grid = Grid::new();
        play_move(&mut grid, 0, 2, Player::Human);
        play_move(&mut grid, 1, 2, Player::Human);
        play_move(&mut grid, 2, 2, Player::Human);
        assert_eq!(grid.get_winner_fast(), Some(Player::Human));
        let mut grid = Grid::new();
        play_move(&mut grid, 0, 0, Player::Human);
        play_move(&mut grid, 1, 1, Player::Human);
        play_move(&mut grid, 2, 2, Player::Human);
        assert_eq!(grid.get_winner_fast(), Some(Player::Human));
        let mut grid = Grid::new();
        play_move(&mut grid, 0, 2, Player::Human);
        play_move(&mut grid, 1, 1, Player::Human);
        play_move(&mut grid, 2, 0, Player::Human);
        assert_eq!(grid.get_winner_fast(), Some(Player::Human));

        // no winner
        let mut grid = Grid::new();
        play_move(&mut grid, 0, 1, Player::Human);
        play_move(&mut grid, 1, 0, Player::Human);
        play_move(&mut grid, 2, 0, Player::Human);
        assert_eq!(grid.get_winner_fast(), None);
    }
}
