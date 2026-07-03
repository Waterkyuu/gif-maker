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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppAction {
    None,
    GenerateGif,
}

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

        if !event::poll(Duration::from_millis(200))? {
            continue;
        }

        let Event::Key(key) = event::read()? else {
            continue;
        };

        match handle_key(app, key.code) {
            AppAction::None => {}
            AppAction::GenerateGif => match generate(app) {
                Ok(_) => {
                    app.message = format!("GIF generated successfully: {}", app.output_file);
                }
                Err(err) => {
                    app.message = format!("Error: {err}");
                }
            },
        }
    }

    Ok(())
}

fn handle_key(app: &mut App, code: KeyCode) -> AppAction {
    // Editing mode owns the keyboard so typed paths never trigger normal shortcuts.
    if app.is_editing_input_dir() {
        match code {
            KeyCode::Enter => {
                app.confirm_input_dir();
            }
            KeyCode::Esc => {
                app.cancel_input_dir_edit();
            }
            KeyCode::Backspace => {
                app.delete_input_dir_character();
            }
            KeyCode::Char(character) => {
                app.push_input_dir_character(character);
            }
            _ => {}
        }

        return AppAction::None;
    }

    match code {
        KeyCode::Char('q') => {
            app.quit();
        }
        KeyCode::Char('i') => {
            app.start_editing_input_dir();
        }
        KeyCode::Up => {
            app.increase_fps();
        }
        KeyCode::Down => {
            app.decrease_fps();
        }
        KeyCode::Enter => {
            return AppAction::GenerateGif;
        }
        _ => {}
    }

    AppAction::None
}

fn generate(app: &App) -> Result<()> {
    let images = file_scanner::scan_images(&app.input_dir)?;

    gif_encoder::create_gif(images, &app.output_file, app.fps)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i_enters_input_directory_edit_mode() {
        let mut app = App::new();

        let action = handle_key(&mut app, KeyCode::Char('i'));

        assert_eq!(action, AppAction::None);
        assert!(app.is_editing_input_dir());
    }

    #[test]
    fn enter_saves_input_directory_while_editing() {
        let mut app = App::new();
        app.start_editing_input_dir();
        replace_input_dir_draft(&mut app, "/tmp/frames");

        let action = handle_key(&mut app, KeyCode::Enter);

        assert_eq!(action, AppAction::None);
        assert_eq!(app.input_dir, "/tmp/frames");
        assert!(!app.is_editing_input_dir());
    }

    #[test]
    fn enter_generates_only_in_normal_mode() {
        let mut app = App::new();

        let action = handle_key(&mut app, KeyCode::Enter);

        assert_eq!(action, AppAction::GenerateGif);
    }

    fn replace_input_dir_draft(app: &mut App, path: &str) {
        for _ in 0..app.input_dir.chars().count() {
            app.delete_input_dir_character();
        }

        for character in path.chars() {
            app.push_input_dir_character(character);
        }
    }
}
