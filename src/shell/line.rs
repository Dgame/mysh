use crate::drawable::Drawable;
use crate::shell;

pub trait XCursor {
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn set_to(&mut self, x: u16);
}

pub trait Line: Drawable {
    fn remove_after(&mut self);
    fn remove_before(&mut self);
    fn cursor(&mut self) -> &mut XCursor;
    fn insert(&mut self, ch: char);
    fn set(&mut self, input: &str);
    fn reset(&mut self) -> Option<String>;
    fn set_padding(&mut self, cursor: &shell::Cursor);
}
