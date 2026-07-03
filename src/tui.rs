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
            Constraint::Length(8),
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

    let (mode, input_style, controls) = if app.is_editing_input_dir() {
        (
            "Editing input folder",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
            "Controls: type path | Backspace delete | Enter save | Esc cancel",
        )
    } else {
        (
            "Ready",
            Style::default().fg(Color::White),
            "Controls: i edit input folder | ↑/↓ FPS | Enter generate | q quit",
        )
    };

    let settings = Paragraph::new(vec![
        Line::from(vec![
            Span::raw("Input Folder:   "),
            Span::styled(app.shown_input_dir(), input_style),
        ]),
        Line::from(vec![
            Span::raw("Output File:     "),
            Span::raw(app.output_file.as_str()),
        ]),
        Line::from(vec![
            Span::raw("FPS:             "),
            Span::raw(app.fps.to_string()),
        ]),
        Line::from(vec![Span::raw("Mode:           "), Span::raw(mode)]),
        Line::from(""),
        Line::from(controls),
    ])
    .block(Block::default().borders(Borders::ALL).title("Settings"));

    frame.render_widget(settings, chunks[1]);

    let message = Paragraph::new(app.message.as_str())
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

    #[test]
    fn draw_shows_input_folder_edit_shortcut() {
        let rendered = render_app(&App::new());

        assert!(rendered.contains("Input Folder:"));
        assert!(rendered.contains("i edit input folder"));
    }

    #[test]
    fn draw_shows_input_folder_editing_state() {
        let mut app = App::new();
        app.start_editing_input_dir();
        replace_input_dir_draft(&mut app, "/tmp/source frames");

        let rendered = render_app(&app);

        assert!(rendered.contains("Input Folder:   /tmp/source frames"));
        assert!(rendered.contains("Mode:           Editing input folder"));
        assert!(rendered.contains("Enter save | Esc cancel"));
    }

    fn char_position(line: &str, needle: &str) -> Option<usize> {
        line.find(needle)
            .map(|byte_position| line[..byte_position].chars().count())
    }

    fn replace_input_dir_draft(app: &mut App, path: &str) {
        for _ in 0..app.input_dir.chars().count() {
            app.delete_input_dir_character();
        }

        for character in path.chars() {
            app.push_input_dir_character(character);
        }
    }

    fn render_app(app: &App) -> String {
        let backend = TestBackend::new(100, 24);
        let mut terminal = Terminal::new(backend).expect("terminal should initialize");

        terminal
            .draw(|frame| {
                draw(frame, app);
            })
            .expect("draw should succeed");

        let buffer = terminal.backend().buffer();
        (0..buffer.area.height)
            .map(|y| {
                (0..buffer.area.width)
                    .filter_map(|x| buffer.cell((x, y)).map(|cell| cell.symbol()))
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
