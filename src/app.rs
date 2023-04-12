#[derive(PartialEq)]
pub enum TileState {
    Nought,
    Cross,
}

pub struct BoardCordinate {
    pub x: usize,
    pub y: usize,
}

impl Default for BoardCordinate {
    fn default() -> Self {
        BoardCordinate { x: 1, y: 1 }
    }
}

#[derive(Default)]
pub struct App {
    pub board: [[Option<TileState>; 3]; 3],
    pub active_tile: BoardCordinate,
    pub is_cross_turn: bool,
    pub game_over: bool,
}

impl App {
    pub fn handle_move(&mut self) {
        if self.is_cross_turn {
            self.board[self.active_tile.y][self.active_tile.x] = Some(TileState::Cross)
        } else {
            self.board[self.active_tile.y][self.active_tile.x] = Some(TileState::Nought)
        }
        self.is_cross_turn = !self.is_cross_turn
    }

    fn check_winner(&mut self, state: &Option<TileState>) -> bool {
        let mut is_winner = false;

        // rows
        for index in 1..self.board.len() {
            if self.board[index].iter().all(|row| row == state) {
                is_winner = true
            }

            // columns
            if &self.board[0][index] == state
                && &self.board[1][index] == state
                && &self.board[2][index] == state
            {
                is_winner = true
            }
        }
        // diagonals
        if (&self.board[0][0] == state && &self.board[1][1] == state && &self.board[2][2] == state)
            || (&self.board[2][0] == state
                && &self.board[1][1] == state
                && &self.board[0][2] == state)
        {
            is_winner = true
        }

        return is_winner;
    }

    pub fn has_won(&mut self) -> bool {
        self.check_winner(&Some(TileState::Cross)) || self.check_winner(&Some(TileState::Nought))
    }
}
