use std::{fmt::Display, str::pattern::Pattern};

struct GameState {
    board: [[CellState; 3]; 3],
    turn: Turn,
}

#[derive(Debug)]
enum Turn {
    X,
    O,
}

#[derive(Clone, Copy)]
enum CellState {
    Empty,
    X,
    O,
}

impl Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CellState::Empty => "_".to_string(),
                CellState::X => "X".to_string(),
                CellState::O => "O".to_string(),
            }
        )
    }
}

fn main() {
    println!("Hello, RC!");

    let mut game_state = GameState {
        turn: Turn::O,
        board: [[CellState::Empty; 3]; 3],
    };

    loop {
        draw(&game_state);
        take_action(&mut game_state);
        if let Some(winner) = check_winner(&game_state) {
            println!("{:?} won!", winner);
            break;
        }
    }
}

fn check_winner(game_state: &GameState) -> Option<Turn> {
    let winning_patterns = [[(0, 0), (0, 1), (0, 2)]];

    let mut result = None;

    for pattern in winning_patterns {
        pattern.iter().all(|(r,c )| {
            
        })
        for (x, y) in pattern {
            
            break
        }
    }

    result
}

fn take_action(game_state: &mut GameState) -> () {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim_end().split(" ").collect::<Vec<&str>>();
    println!("{:?}", input);
    let row = input.get(0).unwrap().parse::<usize>().unwrap();
    let col = input.get(1).unwrap().parse::<usize>().unwrap();
    match game_state.turn {
        Turn::X => {
            game_state.board[row][col] = CellState::X;
            game_state.turn = Turn::O;
        }
        Turn::O => {
            game_state.board[row][col] = CellState::O;
            game_state.turn = Turn::X;
        }
    }

    return ();
}

fn draw(game_state: &GameState) {
    println!("drawing the state");
    for row in game_state.board.iter() {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}
