mod board;
mod game;
mod minimax;

fn main() {
    let b = board::Board::new();
    game::play_game(b);
}

// TODO:
/*
- main/libs separation
- args parsing using clap
- board implementation
    - display- DONE
    - types- DONE
    - 2D array- DONE
    - winner discovery- DONE
    - non constant size- DONE
- game engine:
    - random play
    - minimax
*/
