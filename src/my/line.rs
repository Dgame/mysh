use crate::drawable::Drawable;
use crate::shell::line::XCursor;
use crate::{config, shell};
use log::debug;

struct MyXCursor {
    index: u16,
}

impl MyXCursor {
    fn new() -> Self {
        debug!("Create new XCursor with x = 0");
        Self { index: 0 }
    }
}

impl XCursor for MyXCursor {
    fn move_left(&mut self) {
        self.index -= 1;
        debug!("Move XCursor to the left. x = {}", self.index);
    }

    fn move_right(&mut self) {
        self.index += 1;
        debug!("Move XCursor to the right. x = {}", self.index);
    }
}

pub struct Line {
    input: Vec<char>,
    config: config::Line,
    xcursor: MyXCursor,
    padding: u8,
}

impl Line {
    pub fn new(config: &config::Line) -> Self {
        debug!("New Line: {:?}", config);

        Self {
            input: Vec::with_capacity(config.capacity as usize),
            config: config.clone(),
            xcursor: MyXCursor::new(),
            padding: 0,
        }
    }
}

impl shell::Line for Line {
    fn remove_after(&mut self) {
        if self.xcursor.index > 0 {
            let index = (self.xcursor.index - 1) as usize;
            debug!("Remove after: {}", index);

            self.input.remove(index);
            self.xcursor.move_left();
        }
    }

    fn remove_before(&mut self) {
        let index = self.xcursor.index as usize;
        debug!("Remove before: {}", index);

        self.input.remove(index);
    }

    fn cursor(&mut self) -> &mut XCursor {
        &mut self.xcursor
    }

    fn insert(&mut self, ch: char) {
        let index = self.xcursor.index as usize;
        debug!("Insert {} into Line at index {}", ch, index);

        self.input.insert(index, ch);
        self.xcursor.move_right();
    }

    fn reset(&mut self) {
        debug!("Reset Line");

        self.input.clear();
        self.xcursor = MyXCursor::new();
    }

    fn set_padding(&mut self, cursor: &shell::Cursor) {
        let (x, _) = cursor.get();
        self.padding = x as u8;

        debug!("Set padding to {}", x);
    }
}

impl Drawable for Line {
    fn render_on(&self, term: &mut shell::Terminal) {
        use termion::color;

        debug!("Draw the line");

        let x = (self.padding + self.config.left_padding) as u16;
        term.cursor().set_x(x).clear_after(); // Clean line after Prompt + Padding

        let mut word = String::new();
        for ch in self.input.iter() {
            term.in_color(None).write(*ch);

            if *ch == ' ' {
                word.clear();
            } else {
                word.push(*ch);
                let color: Option<&color::Color> = if word == "for" || word == "foreach" {
                    Some(&color::LightGreen)
                } else {
                    None
                };

                term.cursor().move_left(word.len() as u16).clear_after();
                term.in_color(color).write_text(&word);
            }
        }

        let len = self.input.len() as u16;
        if self.xcursor.index < len {
            term.cursor().move_left(len - self.xcursor.index); // Correct Cursor-Pos after re-draw
        }
    }
}
