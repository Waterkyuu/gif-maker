#[derive(Debug)]
pub struct App {
    pub input_dir: String,
    pub output_file: String,
    pub fps: u16,
    pub messages: String,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            input_dir: String::from("./frames"),
            output_file: String::from("./output.gif"),
            fps: 10,
            messages: String::from("Press enter ton generate GIF"),
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

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}