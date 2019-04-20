use std::io::{Stdout, Write};
use termion::raw::RawTerminal;
use termion::{clear, color, cursor};

pub struct RenderTarget<'a> {
    terminal: &'a mut RawTerminal<Stdout>,
    ypos: u16,
}

pub trait Drawable {
    fn render_on<'a>(&self, target: &mut RenderTarget<'a>);
}

pub trait Shell {
    fn clear(&mut self);
    fn run(&mut self);
    fn render(&mut self);
}

pub struct Cell {
    ch: char,
    fg: Box<color::Color>,
    bg: Box<color::Color>,
}

pub trait Line: Drawable {
    fn remove(&mut self);
    fn insert(&mut self, ch: char);
    fn move_left(&mut self);
    fn move_right(&mut self);
}

pub struct MyLine {
    buffer: String,
    xpos: u16,
}

impl MyLine {
    pub fn new(capacity: u16) -> Self {
        Self {
            buffer: String::with_capacity(capacity as usize),
            xpos: 0,
        }
    }
}

impl Line for MyLine {
    fn remove(&mut self) {
        self.buffer.remove(self.xpos as usize);
        self.move_left();
    }

    fn insert(&mut self, ch: char) {
        self.buffer.insert(self.xpos as usize, ch);
        self.move_right();
    }

    fn move_left(&mut self) {
        self.xpos -= 1;
    }

    fn move_right(&mut self) {
        self.xpos += 1;
    }
}

impl Drawable for MyLine {
    fn render_on<'a>(&self, target: &mut RenderTarget<'a>) {
        let len = self.buffer.len() as u16;
        if self.xpos < len {
            let left = len - self.xpos;
            write!(
                target.terminal,
                "{}{}{}{}",
                cursor::Goto(1, target.ypos),
                clear::CurrentLine,
                self.buffer,
                cursor::Left(left)
            )
                .unwrap();
        } else {
            write!(
                target.terminal,
                "{}{}{}",
                cursor::Goto(1, target.ypos),
                clear::CurrentLine,
                self.buffer
            )
                .unwrap();
        }
        target.terminal.flush().unwrap();
    }
}

pub struct MyShell {
    terminal: RawTerminal<Stdout>,
    line: Box<Line>,
    ypos: u16,
    line_factory: Box<Fn() -> Box<Line>>,
}

impl MyShell {
    pub fn new<F: Fn() -> Box<Line> + 'static>(line_factory: F) -> Self {
        use std::io;
        use termion::raw::IntoRawMode;

        Self {
            terminal: io::stdout().into_raw_mode().unwrap(),
            line: line_factory(),
            ypos: 1,
            line_factory: Box::new(line_factory),
        }
    }

    fn newline(&mut self) {
        self.ypos += 1;
        write!(
            self.terminal,
            "{}{}{}",
            cursor::Goto(1, self.ypos),
            clear::CurrentLine,
            cursor::Down(1)
        )
            .unwrap();
        self.line = (self.line_factory)();
    }
}

impl Shell for MyShell {
    fn clear(&mut self) {
        write!(self.terminal, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
        self.terminal.flush().unwrap();
    }

    fn run(&mut self) {
        use std::io::stdin;
        use termion::event::Key;
        use termion::input::TermRead;

        'L1: loop {
            for key in stdin().keys() {
                match key.unwrap() {
                    Key::Esc => break 'L1,
                    Key::Char(ch) => {
                        if ch == '\n' {
                            self.newline();
                        } else {
                            self.line.insert(ch);
                        }
                    }
                    Key::Backspace => self.line.remove(),
                    Key::Alt(c) => println!("Alt-{}", c),
                    Key::Ctrl(c) => println!("Ctrl-{}", c),
                    Key::Left => self.line.move_left(),
                    Key::Right => self.line.move_right(),
                    Key::Down => println!("<down>"),
                    _ => println!("Other"),
                }

                self.render();
            }
        }
    }

    fn render(&mut self) {
        self.line.render_on(&mut RenderTarget {
            terminal: &mut self.terminal,
            ypos: self.ypos,
        });
        self.terminal.flush().unwrap();
    }
}
