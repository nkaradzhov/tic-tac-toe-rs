use std::fmt::Display;

struct GameState {
    board: [[CellState; 3]; 3],
    turn: Turn,
}

#[derive(Debug)]
enum Turn {
    X,
    O,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum CellState {
    Empty,
    X,
    O,
}

impl TryFrom<&CellState> for Turn {
    type Error = String;

    fn try_from(value: &CellState) -> Result<Self, Self::Error> {
        match value {
            CellState::Empty => Err("could not convert".to_string()),
            CellState::X => Ok(Turn::X),
            CellState::O => Ok(Turn::O)
        }
    }
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
    let winning_patterns = [
        [(0, 0), (0, 1), (0, 2)]
    ];

    let mut result: Option<Turn> = None;

    for pattern in winning_patterns {
        let mut cells_in_pattern = vec![];
        for (x, y) in pattern {
            cells_in_pattern.push(game_state.board[x][y]);
        }

        // {X, X, X} -> set(X)
        // set has size 1 and is not blank, take first, and that is the winner

        vec![CellState::X, CellState::O].iter().for_each(|cell_state| {
            if cells_in_pattern.iter().all(|state| state == cell_state) {
                if let Ok(turn) = Turn::try_from(cell_state) {
                    result = Some(turn);
                }
            }
        });

        if result.is_some() {
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
