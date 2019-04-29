use crate::config::Config;
use crate::drawable::{Cursor, Drawable, RenderTarget, Terminal};
use crate::line::Line;
use crate::prompt::Prompt;
use std::io::Write;
use termion::{clear, cursor};

pub trait Shell {
    fn clear(&mut self);
    fn run(&mut self);
}

pub struct MyShell {
    terminal: Terminal,
    prompt: Prompt,
    line: Box<Line>,
    cursor: Cursor,
}

impl MyShell {
    pub fn new<F: Fn(&crate::config::Line) -> Box<Line> + 'static>(
        config: &Config,
        line_factory: F,
    ) -> Self {
        use std::io;
        use termion::input::MouseTerminal;
        use termion::raw::IntoRawMode;

        Self {
            terminal: MouseTerminal::from(io::stdout().into_raw_mode().unwrap()),
            prompt: Prompt::new(&config.prompt),
            line: line_factory(&config.line),
            cursor: Cursor { x: 0, y: 1 },
        }
    }

    fn newline(&mut self) {
        self.cursor.y += 2;
        self.cursor.x = 0;

        write!(
            self.terminal,
            "{}",
            cursor::Goto(self.cursor.x, self.cursor.y)
        )
        .unwrap();
        self.line = self.line.newline();
    }

    fn render_prompt(&mut self) {
        self.prompt.render_on(&mut RenderTarget {
            terminal: &mut self.terminal,
            cursor: &mut self.cursor,
        });
        self.terminal.flush().unwrap();
    }

    fn render_line(&mut self) {
        self.line.render_on(&mut RenderTarget {
            terminal: &mut self.terminal,
            cursor: &mut self.cursor,
        });
        self.terminal.flush().unwrap();
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
            self.render_prompt();

            for key in stdin().keys() {
                match key.unwrap() {
                    Key::Esc => break 'L1,
                    Key::Char(ch) => {
                        if ch == '\n' {
                            self.newline();
                            self.render_prompt();
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

                self.render_line();
            }
        }
    }
}
