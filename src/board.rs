use std::fmt;
use std::io;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Board {
    state: [[Square; 3]; 3],
    turn: Turn,
}

impl Board {
    pub fn new() -> Self {
        Board {
            state: [
                [Square::Empty, Square::Empty, Square::Empty],
                [Square::Empty, Square::Empty, Square::Empty],
                [Square::Empty, Square::Empty, Square::Empty],
            ],
            turn: Turn::X,
        }
    }
    pub fn from_state(state: [[Square; 3]; 3]) -> Self {
        Board { state, turn: Turn::X }
    }
    fn key_to_coords(key: i8) -> (usize, usize) {
        match key {
            1 => (0, 0),
            2 => (0, 1),
            3 => (0, 2),
            4 => (1, 0),
            5 => (1, 1),
            6 => (1, 2),
            7 => (2, 0),
            8 => (2, 1),
            9 => (2, 2),
            _ => unreachable!()
        }
    }
    fn get_square(&self, i: usize, j: usize) -> Square {
        self.state[i][j]
    }
    fn with_set_square(board: &Board, i: usize, j: usize, square: Square) -> Self {
        let mut b = *board;
        b.state[i][j] = square;
        b
    }
    pub fn try_move(board: &Board, key: i8, turn: Turn) -> Option<Board> {
        match key {
            1..=9 => {
                let (i, j) = Board::key_to_coords(key);
                if board.get_square(i, j) != Square::Empty {
                    None
                } else {
                    Some(Board::with_set_square(board, i, j, turn.to_square()))
                }
            }
            _ => None
        }
    }
    pub fn turn(&self) -> Turn {
        let mut count = 0;
        for row in self.state {
            for col in row {
                match col {
                    Square::Empty => (),
                    _ => count += 1,
                }
            }
        }
        if count % 2 == 0 {
            Turn::X
        } else {
            Turn::O
        }
    }
    pub fn succ(&self, turn: Turn) -> Vec<Board> {
        let mut moves = Vec::new();
        match turn {
            Turn::X => {
                for (i, row) in self.state.iter().enumerate() {
                    for (j, col) in row.iter().enumerate() {
                        match col {
                            Square::Empty => moves.push(Self::with_set_square(&self, i, j, Square::X)),
                            _ => (),
                        }
                    }
                }
            }
            Turn::O => {
                for (i, row) in self.state.iter().enumerate() {
                    for (j, col) in row.iter().enumerate() {
                        match col {
                            Square::Empty => moves.push(Self::with_set_square(&self, i, j, Square::O)),
                            _ => (),
                        }
                    }
                }
            }
        }
        moves
    }
    // Accept if game state is over or if all squares exhausted.
    pub fn accepts(&self) -> bool {
        // Evaluate it to any turn.
        if self.evaluate(Turn::X) != 0 {
            return true;
        } else {
            for row in self.state {
                for col in row {
                    match col {
                        Square::Empty => return false,
                        _ => (),
                    }
                }
            }
            return true;
        }
    }
    pub fn evaluate(&self, turn: Turn) -> i8 {
        if self.row_same(turn.to_square()) || self.col_same(turn.to_square()) || self.diag_same(turn.to_square()) {
            1
        } else if self.row_same(turn.other().to_square()) || self.col_same(turn.other().to_square()) || self.diag_same(turn.other().to_square()) {
            -1
        } else {
            0
        }
    }
    pub fn outcome(&self) -> String {
        if self.accepts() {
            if self.evaluate(Turn::X) == 1 {
                "Player 1 (X) wins!".into()
            } else if self.evaluate(Turn::O) == 1 {
                "Player 2 (O) wins!".into()
            } else {
                "The game is drawn.".into()
            }
        } else {
            "Game over.".into()
        }
    }
    fn row_same(&self, square: Square) -> bool {
        for row in self.state {
            if Self::three_same(row[0], row[1], row[2], square) {
                return true;
            }
        }
        false
    }
    fn col_same(&self, square: Square) -> bool {
        for i in 0..3 {
            if Self::three_same(self.state[0][i], self.state[1][i], self.state[2][i], square) {
                return true;
            }
        }
        false
    }
    fn diag_same(&self, square: Square) -> bool {
        Self::three_same(self.state[0][0], self.state[1][1], self.state[2][2], square) ||
        Self::three_same(self.state[2][0], self.state[1][1], self.state[0][2], square)
    }
    fn three_same(s1: Square, s2: Square, s3: Square, square: Square) -> bool {
        s1 == s2 && s2 == s3 && s3 == square
    }
    // TODO: Might be best to get rid of this dynamic feature, display a guide board at the start, and simply reprimand users if they give an invalid move.
    pub fn display(&self) -> io::Result<()> {
        let mut board_chars = Vec::new();
        let mut pos = 1;
        for row in self.state {
            for col in row {
                match col {
                    Square::X => board_chars.push("X".to_string()),
                    Square::O => board_chars.push("O".to_string()),
                    Square::Empty => {
                        board_chars.push(pos.to_string());
                        pos += 1;
                    },
                }
            }
        }

        use io::Write;
        let mut f = io::stdout();
        writeln!(
            f,
            "{} | {} | {}",
            board_chars[0], board_chars[1], board_chars[2]
        )?;
        writeln!(f, "---------")?;
        writeln!(
            f,
            "{} | {} | {}",
            board_chars[3], board_chars[4], board_chars[5]
        )?;
        writeln!(f, "---------")?;
        writeln!(
            f,
            "{} | {} | {}",
            board_chars[6], board_chars[7], board_chars[8]
        )?;
        Ok(())
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{} | {} | {}",
            self.state[0][0], self.state[0][1], self.state[0][2]
        )?;
        writeln!(f, "---------")?;
        writeln!(
            f,
            "{} | {} | {}",
            self.state[1][0], self.state[1][1], self.state[1][2]
        )?;
        writeln!(f, "---------")?;
        writeln!(
            f,
            "{} | {} | {}",
            self.state[2][0], self.state[2][1], self.state[2][2]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Turn {
    X,
    O,
}

impl Turn {
    pub fn other(&self) -> Self {
        match self {
            Turn::X => Turn::O,
            Turn::O => Turn::X,
        }
    }
    pub fn to_square(&self) -> Square {
        match self {
            Turn::X => Square::X,
            Turn::O => Square::O,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Square {
    X,
    O,
    Empty,
}


impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Square::X => write!(f, "X"),
            Square::O => write!(f, "O"),
            Square::Empty => write!(f, " "),
        }
    }
}
