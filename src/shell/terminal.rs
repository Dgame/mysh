use termion::color;

pub trait Clear {
    fn all(&mut self);
    fn line(&mut self);
}

pub trait Cursor {
    fn set_to(&mut self, x: u16, y: u16) -> &mut Cursor;
    fn get(&self) -> (u16, u16);
    fn clear_after(&mut self) -> &mut Cursor;
    fn set_x(&mut self, x: u16) -> &mut Cursor;
    fn set_y(&mut self, y: u16) -> &mut Cursor;
    fn move_left(&mut self, x: u16) -> &mut Cursor;
    fn move_right(&mut self, x: u16) -> &mut Cursor;
    fn move_up(&mut self, y: u16) -> &mut Cursor;
    fn move_down(&mut self, y: u16) -> &mut Cursor;
}

pub trait Write {
    fn write(&mut self, ch: char);
    fn write_text(&mut self, text: &str);
}

pub trait Terminal {
    fn clear(&mut self) -> &mut Clear;
    fn cursor(&mut self) -> &mut Cursor;
    fn in_color(&mut self, color: Option<&color::Color>) -> &mut Write;
    fn newline(&mut self) -> &mut Terminal;
    fn flush(&mut self) -> &mut Terminal;
}

pub struct CursorPos {
    pub x: u16,
    pub y: u16,
}

impl CursorPos {
    pub fn new() -> Self {
        Self { x: 0, y: 1 }
    }

    pub fn get(&self) -> (u16, u16) {
        (self.x, self.y)
    }
}
