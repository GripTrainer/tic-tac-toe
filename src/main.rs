use crossterm::{
    event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use game::Game;
use std::{error::Error, io, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Color,
    style::Style,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame, Terminal,
};

mod game;

const CELL_HEIGHT: u16 = 3;
const CELL_WIDTH: u16 = 5;
const BOARD_HEIGHT: u16 = 3 * CELL_HEIGHT;
const BOARD_WIDTH: u16 = 3 * CELL_WIDTH;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut game = Game::default();
    loop {
        terminal.draw(|f| ui(f, &mut game))?;

        if poll(Duration::from_millis(500))? {
            match read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Up => game.active_tile.y -= 1,
                    KeyCode::Down => game.active_tile.y += 1,
                    KeyCode::Left => game.active_tile.x -= 1,
                    KeyCode::Right => game.active_tile.x += 1,
                    KeyCode::Enter => game.place_mark(),
                    KeyCode::Char('q') => return Ok(()),
                    _ => {}
                },
                _ => {}
            }
            if game.has_won() {
                break;
            }
        }
    }

    loop {
        terminal.draw(|f| game_over_screen(f, &mut game))?;

        if poll(Duration::from_millis(500))? {
            match read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Char('q') => return Ok(()),
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

fn game_over_screen<B: Backend>(frame: &mut Frame<B>, game: &mut Game) {
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

    horizontal_layout[1]
}

fn ui<B: Backend>(frame: &mut Frame<B>, game: &mut Game) {
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
