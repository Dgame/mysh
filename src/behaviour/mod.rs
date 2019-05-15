use crate::path::OsPath;
use crate::{config, shell};
use termion::color::{Color, Rgb};

pub trait Behaviour {
    fn render(&self, input: &[char], term: &mut shell::Terminal);
}

pub trait Colorizer {
    fn colorize(&self, word: &str, config: &config::Colorize) -> Option<Rgb>;
}

pub struct ExecutableWordColorizer {
    os_path: OsPath,
}

impl ExecutableWordColorizer {
    pub fn new() -> Self {
        Self {
            os_path: OsPath::load(),
        }
    }
}

impl Colorizer for ExecutableWordColorizer {
    fn colorize(&self, word: &str, config: &config::Colorize) -> Option<Rgb> {
        if self.os_path.contains(word) {
            if let Some(ref color) = config.command {
                Some(Rgb(color.red, color.green, color.blue))
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub struct WordColorizeBehaviour {
    config: config::Colorize,
    colorizer: Vec<Box<Colorizer>>,
}

impl WordColorizeBehaviour {
    pub fn new(config: &config::Colorize) -> Self {
        Self {
            config: config.clone(),
            colorizer: Vec::new(),
        }
    }

    pub fn add_colorizer(&mut self, colorizer: Box<Colorizer>) {
        self.colorizer.push(colorizer);
    }

    fn find_color_for(&self, word: &str) -> Option<Rgb> {
        self.colorizer
            .iter()
            .find_map(|colorizer| colorizer.colorize(word, &self.config))
    }
}

impl Behaviour for WordColorizeBehaviour {
    fn render(&self, input: &[char], term: &mut shell::Terminal) {
        let mut word = String::new();
        for ch in input.iter() {
            term.in_color(None).write(*ch);

            if *ch == ' ' {
                word.clear();
            } else {
                word.push(*ch);
                let color = self.find_color_for(&word);
                let color: Option<&Color> = color.as_ref().map(|color| color as &dyn Color);

                term.cursor().move_left(word.len() as u16).clear_after();
                term.in_color(color).write_text(&word);
            }
        }
    }
}
