use std::io::Stdout;
use termion::input::MouseTerminal;
use termion::raw::RawTerminal;

pub type Terminal = MouseTerminal<RawTerminal<Stdout>>;

pub struct Cursor {
    pub x: u16,
    pub y: u16,
}

impl Cursor {
    pub fn get(&self) -> (u16, u16) {
        (self.x, self.y)
    }
}

pub struct RenderTarget<'a> {
    pub terminal: &'a mut Terminal,
    pub cursor: &'a mut Cursor,
}

pub trait Drawable {
    fn render_on(&self, target: &mut RenderTarget);
}
