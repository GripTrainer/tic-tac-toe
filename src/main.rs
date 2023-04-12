use app::App;
use crossterm::{
    event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Color,
    style::Style,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame, Terminal,
};

mod app;

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
    let mut app = App::default();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if poll(Duration::from_millis(500))? {
            match read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Up => app.active_tile.y -= 1,
                    KeyCode::Down => app.active_tile.y += 1,
                    KeyCode::Left => app.active_tile.x -= 1,
                    KeyCode::Right => app.active_tile.x += 1,
                    KeyCode::Enter => app.handle_move(),
                    KeyCode::Char('q') => return Ok(()),
                    _ => {}
                },
                _ => {}
            }

            if app.has_won() {
                if app.is_cross_turn {
                    println!("Noughts has won!")
                } else {
                    println!("Crosses has won!")
                }

                return Ok(());
            }
        }
    }
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

fn ui<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
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
                if row_index == app.active_tile.x && column_index == app.active_tile.y {
                    Style::default().bg(Color::LightYellow)
                } else {
                    Style::default()
                }
            };

            let tile = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick);

            let tile_str = match app.board[column_index][row_index] {
                app::TileState::Empty => "",
                app::TileState::Nought => "O",
                app::TileState::Cross => "X",
            };

            let tile_text = Paragraph::new(tile_str)
                .block(tile)
                .alignment(Alignment::Center)
                .style(style);

            frame.render_widget(tile_text, board_rows[row_index]);
        }
    }
}
