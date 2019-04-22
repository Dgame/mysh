use termion::color;

pub struct Cell {
    pub ch: char,
    pub color: Box<color::Color>,
}

impl Cell {
    pub fn new(ch: char) -> Self {
        Self {
            ch,
            color: Box::new(color::LightWhite),
        }
    }
}
