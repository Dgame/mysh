use crate::cell::Cell;
use crate::drawable::Drawable;
use crate::{config, shell};

pub struct Line {
    cells: Vec<Cell>,
    config: config::Line,
    xpos: u16,
    captured_xpos: u16,
}

impl Line {
    pub fn new(config: &config::Line) -> Self {
        Self {
            cells: Vec::with_capacity(config.capacity as usize),
            config: config.clone(),
            xpos: 0,
            captured_xpos: 0,
        }
    }
}

impl shell::Line for Line {
    fn remove_after(&mut self) {
        if self.xpos > 0 {
            self.cells.remove((self.xpos - 1) as usize);
            self.move_left();
        }
    }

    fn remove_before(&mut self) {
        self.cells.remove(self.xpos as usize);
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

    fn reset(&mut self) {
        self.cells.clear();
        self.xpos = 0;
    }

    fn capture_cursor(&mut self, cursor: &shell::Cursor) {
        let (x, _) = cursor.get();
        self.captured_xpos = x;
    }
}

impl Drawable for Line {
    fn render_on(&self, term: &mut shell::Terminal) {
        term.cursor()
            .set_x(self.captured_xpos + self.config.left_padding)
            .clear_after(); // Clean line after Prompt + Padding
        for cell in self.cells.iter() {
            term.in_color(Some(&*cell.color)).write(cell.ch);
        }

        let len = self.cells.len() as u16;
        if self.xpos < len {
            term.cursor().move_left(len - self.xpos); // Correct Cursor-Pos after re-draw
        }
    }
}
