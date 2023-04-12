use crossterm::{
    event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use game::Game;
use std::{error::Error, io, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use ui::{game_over_screen, game_screen};

mod game;
mod ui;

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
        terminal.draw(|f| game_screen(f, &mut game))?;

        if poll(Duration::from_millis(500))? {
            match read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Up => game.active_tile.y -= 1,
                    KeyCode::Down => game.active_tile.y += 1,
                    KeyCode::Left => game.active_tile.x -= 1,
                    KeyCode::Right => game.active_tile.x += 1,
                    KeyCode::Enter => game.place_mark(),
                    KeyCode::Char(' ') => game.place_mark(),
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
