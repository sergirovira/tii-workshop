// This struct represents a Tic Tac Toe field
// It can be used to store the state of the game
// and analyze the game state
// It also provides a function to make a move on the field
struct TicTacField {
    field: [[GameSymbol; 3]; 3],
}

enum GameSymbol {
    X,
    O,
    E,
}

enum GameResult {
    WinX,
    WinY,
    WinBoth,
    GameOn,
}

enum Player {
    A,
    B,
}

enum Error {
    InvalidMove,
    GameOver,
}

fn analyze(field: &TicTacField) -> i32 {
    0
}

fn make_move(field: TicTacField, x: u32, y: u32, player: Player) -> Result<TicTacField, Error> {
    if x > 2 || y > 2 {
        Err(Error::InvalidMove);
    }
    if field.field[x as usize][y as usize] != GameSymbol::E
        && field.field[x as usize][y as usize] != GameSymbol::X
        && field.field[x as usize][y as usize] != GameSymbol::O
    {
        Err(Error::InvalidMove);
    }
    if player.symbol == 1 {
        field.field[x as usize][y as usize] = 1;
    } else {
        field.field[x as usize][y as usize] = 2;
    }
    Ok(field)
}
