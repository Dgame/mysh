use crate::config::Config;
use crate::drawable::Drawable;
use crate::my;
use crate::shell::line::Line;
use crate::shell::{self, Terminal};

pub struct Shell {
    terminal: my::Terminal,
    line: my::Line,
    prompt: shell::Prompt,
}

impl Shell {
    pub fn new(config: &Config) -> Self {
        Self {
            terminal: my::Terminal::new(),
            line: my::Line::new(&config.line),
            prompt: shell::Prompt::new(&config.prompt),
        }
    }

    fn newline(&mut self) {
        self.terminal.newline();
        self.line.reset();
    }

    fn render_prompt(&mut self) {
        self.prompt.render_on(&mut self.terminal);
        self.line.capture_cursor(self.terminal.cursor());
        self.terminal.flush();
    }

    fn render_line(&mut self) {
        self.line.render_on(&mut self.terminal);
        self.terminal.flush();
    }
}

impl shell::Shell for Shell {
    fn clear(&mut self) {
        self.terminal.clear().all();
        self.terminal.flush();
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
                    Key::Backspace => self.line.remove_after(),
                    Key::Delete => self.line.remove_before(),
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
