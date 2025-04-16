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
        // Check rows and columns
        for i in 0..3 {
            if self.field[i][0] != GameSymbol::E
                && self.field[i][0] == self.field[i][1]
                && self.field[i][1] == self.field[i][2]
            {
                return self.get_winner(self.field[i][0]);
            }
            if self.field[0][i] != GameSymbol::E
                && self.field[0][i] == self.field[1][i]
                && self.field[1][i] == self.field[2][i]
            {
                return self.get_winner(self.field[0][i]);
            }
        }

        // Check diagonals
        if self.field[0][0] != GameSymbol::E
            && self.field[0][0] == self.field[1][1]
            && self.field[1][1] == self.field[2][2]
        {
            return self.get_winner(self.field[0][0]);
        }
        if self.field[0][2] != GameSymbol::E
            && self.field[0][2] == self.field[1][1]
            && self.field[1][1] == self.field[2][0]
        {
            return self.get_winner(self.field[0][2]);
        }

        GameResult::GameOn
    }

    fn get_winner(&self, symbol: GameSymbol) -> GameResult {
        match symbol {
            GameSymbol::X => GameResult::WinX,
            GameSymbol::O => GameResult::WinO,
            GameSymbol::E => GameResult::GameOn,
        }
    }
}
