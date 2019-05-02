pub mod line;
pub mod prompt;
pub mod terminal;
pub mod widget;

pub use self::line::Line;
pub use self::prompt::Prompt;
pub use self::terminal::{Clear, Cursor, CursorPos, Terminal, Write};
pub use self::widget::{Caret, Location, User, Widget};

pub trait Shell {
    fn clear(&mut self);
    fn run(&mut self);
}
