#[derive(Debug)]
pub struct App {
    pub input_dir: String,
    input_dir_draft: String,
    pub output_file: String,
    pub fps: u16,
    pub message: String,
    input_mode: InputMode,
    pub should_quit: bool,
}

const DEFAULT_INPUT_DIR: &str = "examples/frames";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InputMode {
    Normal,
    EditingInputDir,
}

impl App {
    pub fn new() -> Self {
        Self {
            input_dir: DEFAULT_INPUT_DIR.to_owned(),
            input_dir_draft: DEFAULT_INPUT_DIR.to_owned(),
            output_file: String::from("./output.gif"),
            fps: 10,
            message: String::from("Press Enter to generate GIF"),
            input_mode: InputMode::Normal,
            should_quit: false,
        }
    }

    pub fn increase_fps(&mut self) {
        self.fps += 1;
    }

    pub fn decrease_fps(&mut self) {
        if self.fps >= 1 {
            self.fps -= 1;
        }
    }

    pub fn start_editing_input_dir(&mut self) {
        // Keep edits in a draft so Esc can discard them without changing the active folder.
        self.input_dir_draft = self.input_dir.clone();
        self.input_mode = InputMode::EditingInputDir;
        self.message =
            String::from("Editing input directory. Press Enter to save or Esc to cancel.");
    }

    pub fn is_editing_input_dir(&self) -> bool {
        self.input_mode == InputMode::EditingInputDir
    }

    pub fn shown_input_dir(&self) -> &str {
        if self.is_editing_input_dir() {
            &self.input_dir_draft
        } else {
            &self.input_dir
        }
    }

    pub fn push_input_dir_character(&mut self, character: char) {
        if self.is_editing_input_dir() && !character.is_control() {
            self.input_dir_draft.push(character);
        }
    }

    pub fn delete_input_dir_character(&mut self) {
        if self.is_editing_input_dir() {
            self.input_dir_draft.pop();
        }
    }

    pub fn confirm_input_dir(&mut self) {
        if !self.is_editing_input_dir() {
            return;
        }

        // The path is accepted as typed instead of canonicalized, so relative folders stay readable.
        let trimmed = self.input_dir_draft.trim();
        if trimmed.is_empty() {
            self.message = String::from("Input directory cannot be empty.");
            return;
        }

        self.input_dir = trimmed.to_owned();
        self.input_dir_draft = self.input_dir.clone();
        self.input_mode = InputMode::Normal;
        self.message = format!("Input directory set to {}", self.input_dir);
    }

    pub fn cancel_input_dir_edit(&mut self) {
        if self.is_editing_input_dir() {
            self.input_dir_draft = self.input_dir.clone();
            self.input_mode = InputMode::Normal;
            self.message = String::from("Input directory edit canceled.");
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_example_frames_as_default_input_directory() {
        let app = App::new();

        assert_eq!(app.input_dir, "examples/frames");
        assert_eq!(app.shown_input_dir(), "examples/frames");
    }

    #[test]
    fn confirms_edited_input_directory() {
        let mut app = App::new();

        app.start_editing_input_dir();
        replace_input_dir_draft(&mut app, "/tmp/source frames");
        app.confirm_input_dir();

        assert_eq!(app.input_dir, "/tmp/source frames");
        assert!(!app.is_editing_input_dir());
    }

    #[test]
    fn cancels_input_directory_edits() {
        let mut app = App::new();

        app.start_editing_input_dir();
        replace_input_dir_draft(&mut app, "/tmp/other");
        app.cancel_input_dir_edit();

        assert_eq!(app.input_dir, "examples/frames");
        assert!(!app.is_editing_input_dir());
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
