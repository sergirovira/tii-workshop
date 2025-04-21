#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GameSymbol {
    X,
    O,
    E, // Empty
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GameResult {
    WinX,
    WinO,
    GameOn,
}

pub enum Player {
    A,
    B,
}

pub struct TicTacField {
    field: [[GameSymbol; 3]; 3],
}

impl Default for TicTacField {
    fn default() -> Self {
        Self::new()
    }
}

impl TicTacField {
    pub fn new() -> Self {
        Self {
            field: [[GameSymbol::E; 3]; 3],
        }
    }

    pub fn print(&self) {
        for row in &self.field {
            for &symbol in row {
                print!(
                    "{} ",
                    match symbol {
                        GameSymbol::X => "X",
                        GameSymbol::O => "O",
                        GameSymbol::E => ".",
                    }
                );
            }
            println!();
        }
    }

    /// Makes a move for the given player at the specified coordinates.
    pub fn make_move(&mut self, x: usize, y: usize, player: &Player) -> Result<(), &'static str> {
        if x >= 3 || y >= 3 {
            return Err("Invalid move: Out of bounds");
        }
        if self.field[x][y] != GameSymbol::E {
            return Err("Invalid move: Cell already occupied");
        }

        self.field[x][y] = match player {
            Player::A => GameSymbol::X,
            Player::B => GameSymbol::O,
        };
        Ok(())
    }

    pub fn analyze(&self) -> GameResult {
        let lines = [
            // Rows
            [self.field[0][0], self.field[0][1], self.field[0][2]],
            [self.field[1][0], self.field[1][1], self.field[1][2]],
            [self.field[2][0], self.field[2][1], self.field[2][2]],
            // Columns
            [self.field[0][0], self.field[1][0], self.field[2][0]],
            [self.field[0][1], self.field[1][1], self.field[2][1]],
            [self.field[0][2], self.field[1][2], self.field[2][2]],
            // Diagonals
            [self.field[0][0], self.field[1][1], self.field[2][2]],
            [self.field[0][2], self.field[1][1], self.field[2][0]],
        ];

        for line in &lines {
            if *line == [GameSymbol::X; 3] {
                return GameResult::WinX;
            }
            if *line == [GameSymbol::O; 3] {
                return GameResult::WinO;
            }
        }

        GameResult::GameOn
    }
}
