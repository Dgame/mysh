use crate::shell::{self, CursorPos};
use log::debug;
use std::io::{self, Stdout, Write};
use termion::input::MouseTerminal;
use termion::raw::RawTerminal;
use termion::{clear, color, cursor};

type Term = MouseTerminal<RawTerminal<Stdout>>;

pub struct Terminal {
    terminal: Term,
    cursor_pos: CursorPos,
}

impl Terminal {
    pub fn new() -> Self {
        use termion::raw::IntoRawMode;

        Self {
            terminal: Term::from(io::stdout().into_raw_mode().unwrap()),
            cursor_pos: CursorPos::new(),
        }
    }
}

impl shell::Write for Terminal {
    fn write(&mut self, ch: char) {
        debug!("Write {}", ch);
        write!(self.terminal, "{}", ch).unwrap();

        self.cursor_pos.x += 1;
    }

    fn write_text(&mut self, text: &str) {
        debug!("Write {}", text);
        write!(
            self.terminal,
            "{text}{reset}",
            text = text,
            reset = color::Fg(color::Reset)
        )
        .unwrap();

        self.cursor_pos.x += text.len() as u16;
    }
}

impl shell::Terminal for Terminal {
    fn clear(&mut self) -> &mut shell::Clear {
        self
    }

    fn cursor(&mut self) -> &mut shell::Cursor {
        self
    }

    fn in_color(&mut self, color: Option<&color::Color>) -> &mut shell::Write {
        if let Some(color) = color {
            debug!("With color");
            write!(self.terminal, "{color}", color = color::Fg(color)).unwrap(); // TODO: Reset vor {color}?
        } else {
            debug!("Without color");
            write!(self.terminal, "{reset}", reset = color::Fg(color::Reset)).unwrap(); // TODO: Reset vor {color}?
        }

        self
    }

    fn newline(&mut self) -> &mut shell::Terminal {
        self.cursor_pos.x = 0;
        self.cursor_pos.y += 1;

        let (x, y) = self.cursor_pos.get();
        debug!("Newline: x = {}, y = {}", x, y);
        write!(self.terminal, "{}", cursor::Goto(x, y)).unwrap();

        self
    }

    fn flush(&mut self) -> &mut shell::Terminal {
        debug!("Flush");
        self.terminal.flush().unwrap();

        self
    }
}

impl shell::Clear for Terminal {
    fn all(&mut self) {
        debug!("Clear All!");
        write!(self.terminal, "{}{}", clear::All, cursor::Goto(0, 1)).unwrap();
        self.cursor_pos = CursorPos::new();
    }

    fn line(&mut self) {
        self.cursor_pos.x = 0;
        let (x, y) = self.cursor_pos.get();

        debug!("Clear this Line: x = {}, y = {}", x, y);
        write!(
            self.terminal,
            "{}{}",
            clear::CurrentLine,
            cursor::Goto(x, y)
        )
        .unwrap();
    }
}

impl shell::Cursor for Terminal {
    fn set_to(&mut self, x: u16, y: u16) -> &mut shell::Cursor {
        debug!("Set Cursor to x = {} y = {}", x, y);

        self.cursor_pos.x = x;
        self.cursor_pos.y = y;

        //        write!(self.terminal, "{}", cursor::Goto(x, y)).unwrap(); // TODO: Use or drop?

        self
    }

    fn get(&self) -> (u16, u16) {
        self.cursor_pos.get()
    }

    fn clear_after(&mut self) -> &mut shell::Cursor {
        let (x, y) = self.cursor_pos.get();
        debug!("Clear after x = {}, y = {}", x, y);
        write!(
            self.terminal,
            "{}{}",
            cursor::Goto(x, y),
            clear::AfterCursor
        )
        .unwrap();

        self
    }

    fn set_x(&mut self, x: u16) -> &mut shell::Cursor {
        debug!("Set x to {}", x);

        self.cursor_pos.x = x;

        self
    }

    fn set_y(&mut self, y: u16) -> &mut shell::Cursor {
        debug!("Set y to {}", y);

        self.cursor_pos.y = y;

        self
    }

    fn move_left(&mut self, x: u16) -> &mut shell::Cursor {
        debug!("Move Cursor {} left", x);

        if self.cursor_pos.x > x {
            self.cursor_pos.x -= x;
        } else {
            self.cursor_pos.x = 0;
        }

        write!(self.terminal, "{}", cursor::Left(x)).unwrap();

        self
    }

    fn move_right(&mut self, x: u16) -> &mut shell::Cursor {
        debug!("Move Cursor {} right", x);

        self.cursor_pos.x += x;
        write!(self.terminal, "{}", cursor::Right(x)).unwrap();

        self
    }

    fn move_up(&mut self, y: u16) -> &mut shell::Cursor {
        debug!("Move Cursor {} up", y);

        self.cursor_pos.y -= y;
        write!(self.terminal, "{}", cursor::Up(y)).unwrap();

        self
    }

    fn move_down(&mut self, y: u16) -> &mut shell::Cursor {
        debug!("Move Cursor {} down", y);

        self.cursor_pos.y += y;
        write!(self.terminal, "{}", cursor::Down(y)).unwrap();

        self
    }
}
