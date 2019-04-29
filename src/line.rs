use crate::cell::Cell;
use crate::config;
use crate::drawable::{Drawable, RenderTarget};

pub trait Line: Drawable {
    fn remove(&mut self);
    fn insert(&mut self, ch: char);
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn newline(&self) -> Box<Line>;
}

pub struct MyLine {
    cells: Vec<Cell>,
    xpos: u16,
    config: config::Line,
}

impl MyLine {
    pub fn new(config: &config::Line) -> Self {
        Self {
            cells: Vec::with_capacity(config.capacity as usize),
            xpos: 0,
            config: config.clone(),
        }
    }
}

impl Line for MyLine {
    fn remove(&mut self) {
        if self.xpos > 0 {
            self.cells.remove((self.xpos - 1) as usize);
            self.move_left();
        }
    }

    fn insert(&mut self, ch: char) {
        self.cells.insert(self.xpos as usize, Cell::new(ch));
        self.move_right();
    }

    fn move_left(&mut self) {
        self.xpos -= 1;
    }

    fn move_right(&mut self) {
        self.xpos += 1;
    }

    fn newline(&self) -> Box<Line> {
        Box::new(Self::new(&self.config))
    }
}

impl Drawable for MyLine {
    fn render_on(&self, target: &mut RenderTarget) {
        use std::io::Write;
        use termion::{clear, color, cursor};

        let (x, y) = target.cursor.get();

        write!(
            target.terminal,
            "{}{}",
            cursor::Goto(x, y),
            clear::AfterCursor,
        )
        .unwrap();
        for cell in self.cells.iter() {
            write!(
                target.terminal,
                "{color}{ch}{reset}",
                color = color::Fg(&*cell.color),
                ch = cell.ch,
                reset = color::Fg(color::Reset)
            )
            .unwrap();
        }

        let len = self.cells.len() as u16;
        if self.xpos < len {
            let left = len - self.xpos;
            write!(target.terminal, "{}", cursor::Left(left)).unwrap();
        }
    }
}
