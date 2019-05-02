use crate::drawable::Drawable;
use crate::shell;

pub trait Line: Drawable {
    fn remove_after(&mut self);
    fn remove_before(&mut self);
    fn insert(&mut self, ch: char);
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn reset(&mut self);
    fn capture_cursor(&mut self, cursor: &shell::Cursor);
}
