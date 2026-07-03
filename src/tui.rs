use crate::app::App;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

const GIF_MAKER_LOGO: &str = r#" ██████╗ ██╗███████╗    ███╗   ███╗ █████╗ ██╗  ██╗███████╗██████╗ 
██╔════╝ ██║██╔════╝    ████╗ ████║██╔══██╗██║ ██╔╝██╔════╝██╔══██╗
██║  ███╗██║█████╗      ██╔████╔██║███████║█████╔╝ █████╗  ██████╔╝
██║   ██║██║██╔══╝      ██║╚██╔╝██║██╔══██║██╔═██╗ ██╔══╝  ██╔══██╗
╚██████╔╝██║██║         ██║ ╚═╝ ██║██║  ██║██║  ██╗███████╗██║  ██║
 ╚═════╝ ╚═╝╚═╝         ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝"#;

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),
            Constraint::Length(7),
            Constraint::Min(3),
        ])
        .split(frame.area());

    let title = Paragraph::new(GIF_MAKER_LOGO)
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("GIF Maker")
                .title_alignment(Alignment::Center),
        );

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

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{Terminal, backend::TestBackend};

    #[test]
    fn draw_renders_gif_maker_logo_banner() {
        let backend = TestBackend::new(90, 24);
        let mut terminal = Terminal::new(backend).expect("terminal should initialize");
        let app = App::new();

        terminal
            .draw(|frame| {
                draw(frame, &app);
            })
            .expect("draw should succeed");

        let buffer = terminal.backend().buffer();
        let rendered = (0..buffer.area.height)
            .map(|y| {
                (0..buffer.area.width)
                    .filter_map(|x| buffer.cell((x, y)).map(|cell| cell.symbol()))
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        assert!(rendered.contains(" ██████╗ ██╗███████╗"));
        assert!(rendered.contains("██╔════╝ ██║██╔════╝"));
        assert!(rendered.contains("GIF Maker"));
    }

    #[test]
    fn draw_centers_gif_maker_banner() {
        let backend = TestBackend::new(90, 24);
        let mut terminal = Terminal::new(backend).expect("terminal should initialize");
        let app = App::new();

        terminal
            .draw(|frame| {
                draw(frame, &app);
            })
            .expect("draw should succeed");

        let buffer = terminal.backend().buffer();
        let lines = (0..buffer.area.height)
            .map(|y| {
                (0..buffer.area.width)
                    .filter_map(|x| buffer.cell((x, y)).map(|cell| cell.symbol()))
                    .collect::<String>()
            })
            .collect::<Vec<_>>();

        assert_eq!(char_position(&lines[0], "GIF Maker"), Some(40));
        assert_eq!(char_position(&lines[1], " ██████╗ ██╗███████╗"), Some(12));
    }

    fn char_position(line: &str, needle: &str) -> Option<usize> {
        line.find(needle)
            .map(|byte_position| line[..byte_position].chars().count())
    }
}
