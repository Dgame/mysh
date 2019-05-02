use crate::shell;

pub trait Drawable {
    fn render_on(&self, terminal: &mut shell::Terminal);
}
