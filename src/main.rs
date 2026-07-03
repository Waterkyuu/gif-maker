mod app;
mod file_scanner;
mod gif_encoder;
mod tui;

use anyhow::Result;
use app::App;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{io, time::Duration};

fn main() -> Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    let result = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|frame| {
            tui::draw(frame, app);
        })?;

        if app.should_quit {
            break;
        }

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        app.quit();
                    }
                    KeyCode::Up => {
                        app.increase_fps();
                    }
                    KeyCode::Down => {
                        app.decrease_fps();
                    }
                    KeyCode::Enter => match generate(app) {
                        Ok(_) => {
                            app.message =
                                format!("GIF generated successfully: {}", app.output_file);
                        }
                        Err(err) => {
                            app.message = format!("Error: {err}");
                        }
                    },
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

fn generate(app: &App) -> Result<()> {
    let images = file_scanner::scan_images(&app.input_dir)?;

    gif_encoder::create_gif(images, &app.output_file, app.fps)?;

    Ok(())
}
