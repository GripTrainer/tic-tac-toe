type Board = [[Option<Player>; 3]; 3];

pub struct BoardCordinate {
    pub x: usize,
    pub y: usize,
}

impl Default for BoardCordinate {
    fn default() -> Self {
        BoardCordinate { x: 1, y: 1 }
    }
}

#[derive(Default, PartialEq, Clone, Copy)]
pub enum Player {
    #[default]
    Cross,
    Nought,
}

#[derive(PartialEq, Clone, Copy)]
pub enum FinishedState {
    Draw,
    Won(Player),
}

impl FinishedState {
    fn new(player: Option<Player>) -> Self {
        match player {
            Some(player) => FinishedState::Won(player),
            None => FinishedState::Draw,
        }
    }
}

#[derive(Default)]
pub struct Game {
    pub board: Board,
    pub active_tile: BoardCordinate,
    pub player_turn: Player,
}

impl Game {
    pub fn place_mark(&mut self) {
        let active_tile = &self.board[self.active_tile.y][self.active_tile.x];
        match active_tile {
            None => match self.player_turn {
                Player::Cross => {
                    self.board[self.active_tile.y][self.active_tile.x] = Some(Player::Cross);
                    self.player_turn = Player::Nought;
                }
                Player::Nought => {
                    self.board[self.active_tile.y][self.active_tile.x] = Some(Player::Nought);
                    self.player_turn = Player::Cross;
                }
            },
            Some(_) => {}
        }
    }

    pub fn check_finish_state(&mut self) -> Option<FinishedState> {
        if check_winner(&self.board, Some(Player::Cross)) {
            Some(FinishedState::new(Some(Player::Cross)))
        } else if check_winner(&self.board, Some(Player::Nought)) {
            Some(FinishedState::new(Some(Player::Nought)))
        } else if self
            .board
            .iter()
            .all(|row| row.iter().all(|col| col.is_some()))
        {
            Some(FinishedState::new(None))
        } else {
            None
        }
    }
}

fn check_winner(board: &Board, state: Option<Player>) -> bool {
    let mut is_winner = false;

    // rows
    for index in 0..board.len() {
        if board[index].iter().all(|row| row == &state) {
            is_winner = true
        }

        // columns
        if board[0][index] == state && board[1][index] == state && board[2][index] == state {
            is_winner = true
        }
    }
    // diagonals
    if (board[0][0] == state && board[1][1] == state && board[2][2] == state)
        || (board[2][0] == state && board[1][1] == state && board[0][2] == state)
    {
        is_winner = true
    }

    return is_winner;
}

#[cfg(test)]
mod tests {
    use super::{check_winner, Board, Player};

    #[test]
    fn test_check_winner_rows() {
        let mut board = Board::default();
        let player = Player::Cross;
        board[0][0] = Some(Player::Cross);
        board[0][1] = Some(Player::Cross);
        board[0][2] = Some(Player::Cross);

        let should_win = check_winner(&board, Some(player));

        assert!(should_win);
    }
}
