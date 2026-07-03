use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(7),
            Constraint::Min(3),
        ])
        .split(frame.area());

    let title = Paragraph::new("Rust GIF Maker TUI")
        .block(Block::default().borders(Borders::ALL).title("Title"));

    frame.render_widget(title, chunks[0]);

    let settings = Paragraph::new(vec![
        Line::from(vec![
            Span::raw("Input Directory: "),
            Span::raw(app.input_dir.clone()),
        ]),
        Line::from(vec![
            Span::raw("Output File:     "),
            Span::raw(app.output_file.clone()),
        ]),
        Line::from(vec![
            Span::raw("FPS:             "),
            Span::raw(app.fps.to_string()),
        ]),
        Line::from(""),
        Line::from("Controls: ↑ increase FPS | ↓ decrease FPS | Enter generate | q quit"),
    ])
    .block(Block::default().borders(Borders::ALL).title("Settings"));

    frame.render_widget(settings, chunks[1]);

    let message = Paragraph::new(app.message.clone())
        .block(Block::default().borders(Borders::ALL).title("Message"));

    frame.render_widget(message, chunks[2]);
}
