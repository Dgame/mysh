use crate::behaviour::InputBehaviour;
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
    behaviour: Vec<Box<InputBehaviour>>,
    padding: u8,
}

impl Line {
    pub fn new(config: &config::Line) -> Self {
        debug!("New Line: {:?}", config);

        Self {
            input: Vec::with_capacity(config.capacity as usize),
            config: config.clone(),
            xcursor: MyXCursor::new(),
            behaviour: Vec::new(),
            padding: 0,
        }
    }

    pub fn add_behaviour(&mut self, behaviour: Box<InputBehaviour>) {
        self.behaviour.push(behaviour);
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

    fn reset(&mut self) -> Option<String> {
        debug!("Reset Line");

        let input: String = self.input.iter().collect();
        self.input.clear();
        self.xcursor = MyXCursor::new();

        if input.is_empty() {
            None
        } else {
            Some(input)
        }
    }

    fn set_padding(&mut self, cursor: &shell::Cursor) {
        let (x, _) = cursor.get();
        self.padding = x as u8;

        debug!("Set padding to {}", x);
    }
}

impl Drawable for Line {
    fn render_on(&self, term: &mut shell::Terminal) {
        debug!("Draw the line");

        let x = u16::from(self.padding + self.config.left_padding);
        term.cursor().set_x(x).clear_after(); // Clean line after Prompt + Padding

        for behaviour in self.behaviour.iter() {
            behaviour.render(&self.input, term);
        }

        let len = self.input.len() as u16;
        if self.xcursor.index < len {
            term.cursor().move_left(len - self.xcursor.index); // Correct Cursor-Pos after re-draw
        }
    }
}
