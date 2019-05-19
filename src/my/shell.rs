use crate::config::Config;
use crate::drawable::Drawable;
use crate::history::History;
use crate::shell::line::Line;
use crate::shell::{self, Terminal};

pub struct Shell {
    terminal: Box<Terminal>,
    line: Box<Line>,
    prompt: shell::Prompt,
    history: History,
}

impl Shell {
    pub fn new(config: &Config, terminal: Box<Terminal>, line: Box<Line>) -> Self {
        Self {
            terminal,
            line,
            prompt: shell::Prompt::new(&config.prompt),
            history: History::load(&config.history),
        }
    }

    fn newline(&mut self) {
        self.terminal.newline();
        if let Some(input) = self.line.reset() {
            self.history.insert(input);
        }
    }

    fn render_prompt(&mut self) {
        self.prompt.render_on(&mut *self.terminal);
        self.line.set_padding(self.terminal.cursor());
        self.terminal.flush();
    }

    fn render_line(&mut self) {
        self.line.render_on(&mut *self.terminal);
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
                    Key::Left => self.line.cursor().move_left(),
                    Key::Right => self.line.cursor().move_right(),
                    Key::Down => println!("<down>"),
                    _ => println!("Other"),
                }

                self.render_line();
            }
        }
    }
}
