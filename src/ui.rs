use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Color,
    style::Style,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::game::{self, Game};

const CELL_HEIGHT: u16 = 3;
const CELL_WIDTH: u16 = 5;
const BOARD_HEIGHT: u16 = 3 * CELL_HEIGHT;
const BOARD_WIDTH: u16 = 3 * CELL_WIDTH;

pub fn game_over_screen<B: Backend>(frame: &mut Frame<B>, game: &mut Game) {
    let size = frame.size();
    let game_border = Block::default()
        .borders(Borders::ALL)
        .title("Tic Tac Toe")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    frame.render_widget(game_border, frame.size());

    let exit_message_height = 3;
    let vertical_padding = (size.height - exit_message_height) / 2;

    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(vertical_padding),
                Constraint::Length(exit_message_height),
                Constraint::Length(vertical_padding),
            ]
            .as_ref(),
        )
        .split(size);

    let game_over_text_area = vertical_layout[1];
    // Winner is opposite to who's turn it is
    // TODO: Set a winner attribute on the game struct
    let winner_name = match game.player_turn {
        game::Player::Cross => "O",
        game::Player::Nought => "X",
    };

    let game_over_message = format!("{} is the winner. <q> to exit", winner_name);

    let tile = Block::default().borders(Borders::NONE);

    let tile_text = Paragraph::new(game_over_message)
        .block(tile)
        .alignment(Alignment::Center);

    frame.render_widget(tile_text, game_over_text_area);
}

fn create_board_rectangle<B: Backend>(frame: &mut Frame<B>) -> Rect {
    let size = frame.size();
    let vertical_padding = (size.height - BOARD_HEIGHT) / 2;
    let horizontal_padding = (size.width - BOARD_WIDTH) / 2;

    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(vertical_padding),
                Constraint::Length(BOARD_HEIGHT),
                Constraint::Length(vertical_padding),
            ]
            .as_ref(),
        )
        .split(size);

    let horizontal_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(horizontal_padding),
                Constraint::Length(BOARD_WIDTH),
                Constraint::Length(horizontal_padding),
            ]
            .as_ref(),
        )
        .split(vertical_layout[1]);

    let bottom_center_bottom = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(vertical_layout[2]);

    let instruction_menu = Block::default()
        .borders(Borders::TOP)
        .title(" move: <arrows> | place: <spacebar> or <enter> | quit: <q> ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Plain);

    frame.render_widget(instruction_menu, bottom_center_bottom[1]);

    let center_middle_chunk = horizontal_layout[1];

    center_middle_chunk
}

pub fn game_screen<B: Backend>(frame: &mut Frame<B>, game: &mut Game) {
    let game_border = Block::default()
        .borders(Borders::ALL)
        .title("Tic Tac Toe")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    frame.render_widget(game_border, frame.size());

    let board = create_board_rectangle(frame);

    let board_columns = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(CELL_HEIGHT),
                Constraint::Length(CELL_HEIGHT),
                Constraint::Length(CELL_HEIGHT),
            ]
            .as_ref(),
        )
        .split(board);

    for column_index in 0..board_columns.len() {
        let board_rows = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Length(CELL_WIDTH),
                    Constraint::Length(CELL_WIDTH),
                    Constraint::Length(CELL_WIDTH),
                ]
                .as_ref(),
            )
            .split(board_columns[column_index]);

        for row_index in 0..board_rows.len() {
            let style = {
                if row_index == game.active_tile.x && column_index == game.active_tile.y {
                    Style::default().bg(Color::LightYellow)
                } else {
                    Style::default()
                }
            };

            let tile = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick);

            let tile_str = match game.board[column_index][row_index] {
                None => "",
                Some(game::Player::Nought) => "O",
                Some(game::Player::Cross) => "X",
            };

            let tile_text = Paragraph::new(tile_str)
                .block(tile)
                .alignment(Alignment::Center)
                .style(style);

            frame.render_widget(tile_text, board_rows[row_index]);
        }
    }
}
