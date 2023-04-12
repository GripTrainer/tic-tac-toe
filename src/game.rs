#[derive(Default, PartialEq)]
pub enum Player {
    #[default]
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
pub struct Game {
    pub board: [[Option<Player>; 3]; 3],
    pub active_tile: BoardCordinate,
    pub player_turn: Player,
}

impl Game {
    pub fn place_mark(&mut self) {
        match self.player_turn {
            Player::Cross => {
                self.board[self.active_tile.y][self.active_tile.x] = Some(Player::Cross);
                self.player_turn = Player::Nought;
            }
            Player::Nought => {
                self.board[self.active_tile.y][self.active_tile.x] = Some(Player::Nought);
                self.player_turn = Player::Cross;
            }
        }
    }

    fn check_winner(&mut self, state: &Option<Player>) -> bool {
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
        self.check_winner(&Some(Player::Cross)) || self.check_winner(&Some(Player::Nought))
    }
}
