use serde_derive::Deserialize;
use crossterm::{
    ClearType, Color, InputEvent, KeyEvent, RawScreen, Terminal, TerminalCursor, TerminalInput,
};
use std::rc::Rc;

trait Drawable {
    fn render_on(&self, shell: &Shell);
}

struct Cell {
    ch: char,
    fg: Color,
    bg: Color,
}

impl Cell {
    fn new(ch: char) -> Self {
        Self {
            ch,
            fg: Color::Grey,
            bg: Color::Black,
        }
    }

    fn colorize(&mut self, fg: Option<Color>, bg: Option<Color>) {
        if let Some(fg) = fg {
            self.fg = fg;
        }

        if let Some(bg) = bg {
            self.bg = bg;
        }
    }
}

impl Drawable for Cell {
    fn render_on(&self, shell: &Shell) {
        unimplemented!()
    }
}

pub trait Shell {
    fn get_terminal(&self) -> &Terminal;
    fn get_input(&self) -> &TerminalInput;
    fn get_cursor(&mut self) -> &mut TerminalCursor;
    fn render(&mut self);
    fn draw(&self, d: &Drawable);
}

#[derive(Default, Clone)]
struct Position {
    x: u16,
    y: u16,
}

impl Position {
    fn get(&self) -> (u16, u16) {
        (self.x, self.y)
    }
}

#[derive(Deserialize)]
struct Config {
    pub prompt: String,
    pub line_length: usize,
}

fn fmt_prompt(prompt: &str) -> String {
    use std::env;

    if prompt.is_empty() {
        String::from("$ ")
    } else {
        prompt
            .replace("{user}", whoami::username().as_str())
            .replace("{host}", whoami::hostname().as_str())
            .replace(
                "{dir}",
                env::current_dir()
                    .map(|dir| dir.display().to_string())
                    .unwrap_or(String::new())
                    .as_str(),
            )
    }
}

struct Prompt {
    lines: Vec<String>,
}

impl Prompt {
    fn create_from(config: &Config) -> Self {
        Self {
            lines: fmt_prompt(&config.prompt)
                .lines()
                .map(|line| line.trim_start())
                .filter(|line| line.is_empty())
                .map(|line| line.to_string())
                .collect(),
        }
    }
}

impl Drawable for Prompt {
    fn render_on(&self, shell: &Shell) {
        let term = shell.get_terminal();
        //let mut cursor = shell.get_cursor();

        for line in self.lines.iter() {
            term.write(line);
            //cursor.move_down(1);
            //cursor.goto(0, 0);
        }
    }
}

struct Line {
    buffer: String,
    capacity: usize,
    //position: Rc<Position>,
}

impl Line {
    fn with_capacity(capacity: usize/*, position: Rc<Position>*/) -> Self {
        Self {
            buffer: String::with_capacity(capacity),
            //position,
            capacity,
        }
    }

    fn insert_at(&mut self, idx: usize, ch: char) {
        self.buffer.insert(idx, ch);
        //self.position.x += 1;
    }
/*
    fn insert_at_x(&mut self, ch: char) {
        self.insert_at(self.position.x as usize, ch);
    }
*/
    fn remove_at(&mut self, idx: usize) {
        self.buffer.remove(idx);
    }
/*
    fn remove_at_x(&mut self) {
        self.remove_at(self.position.x as usize)
    }
*/
    fn len(&self) -> usize {
        self.buffer.len()
    }

    fn new(&self) -> Self {
        Self::with_capacity(self.capacity/*, self.position.clone()*/)
    }
}

impl Drawable for Line {
    fn render_on(&self, shell: &Shell) {
        for ch in self.buffer.chars() {
            shell.get_terminal().write(ch);
        }
    }
}

fn read_config() -> Result<Config, String> {
    use std::io::Read;

    let mut file = std::fs::File::open("mysh.toml").map_err(|e| e.to_string())?;
    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|e| e.to_string())?;

    toml::from_str(&content).map_err(|e| e.to_string())
}

pub struct MyShell {
    terminal: Terminal,
    cursor: TerminalCursor,
    input: TerminalInput,
    prompt: Prompt,
    line: Line,
    position: Position, //Rc<Position>,
    config: Config,
}

impl MyShell {
    pub fn new() -> Self {
        let config = read_config().expect("Could not load mysh.toml");
        RawScreen::into_raw_mode().unwrap();

        let position = Position::default();//Rc::new(Position::default());

        Self {
            terminal: crossterm::terminal(),
            cursor: crossterm::cursor(),
            input: crossterm::input(),
            line: Line::with_capacity(config.line_length/*, position.clone()*/),
            position,
            prompt: Prompt::create_from(&config),
            config,
        }
    }

    pub fn clear(&self) {
        self.terminal.clear(ClearType::All);
    }

    pub fn execute(&mut self) {
        let mut stdin = self.input.read_sync();

        loop {
            self.render();

            if let Some(event) = stdin.next() {
                if let InputEvent::Keyboard(key_event) = event {
                    match key_event {
                        KeyEvent::Char(ch) => {
                            if ch == '\n' {
                                self.cursor.move_down(1);
                                self.position.y += 1;

                                self.line = self.line.new();
                            } else {
                                self.line.insert_at(self.position.x as usize, ch);
                                self.position.x += 1;
                            }
                        }
                        KeyEvent::Backspace => {
                            if self.position.x > 0 {
                                self.position.x -= 1;
                                //self.line.remove_at_x();
                                self.line.remove_at(self.position.x as usize);
                            }
                        }
                        KeyEvent::Delete => {
                            if self.position.x > 0 && (self.position.x as usize) < self.line.len() {
                                //self.line.remove_at_x();
                                self.line.remove_at(self.position.x as usize);
                            }
                        }
                        KeyEvent::Left => {
                            self.cursor.move_left(1);
                            self.position.x -= 1;
                        }
                        KeyEvent::Esc => break,
                        _ => {
                            dbg!(key_event);
                        }
                    }
                }
            }
        }
    }
}

impl Shell for MyShell {
    fn get_terminal(&self) -> &Terminal {
        &self.terminal
    }

    fn get_input(&self) -> &TerminalInput {
        &self.input
    }

    fn get_cursor(&mut self) -> &mut TerminalCursor {
        &mut self.cursor
    }

    fn render(&mut self) {
        let (x, y) = self.position.get();

        self.cursor.goto(0, y);
        self.terminal.clear(ClearType::CurrentLine);

        self.draw(&self.prompt);
        self.draw(&self.line);

        if (x as usize) != self.line.len() {
            self.cursor.goto(x as u16, y);
        }
    }

    fn draw(&self, d: &Drawable) {
        d.render_on(self);
    }
}
