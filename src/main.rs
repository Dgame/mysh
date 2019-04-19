use crossterm::{Color, RawScreen};

struct Cell {
    ch: char,
    fg: Color,
    bg: Color
}

impl Cell {
    fn new(ch: char) -> Self {
        Self { ch, fg: Color::Grey, bg: Color::Black }
    }
}

fn main() {
    use crossterm::InputEvent;
    use crossterm::KeyEvent;
    use crossterm::ClearType;

    let screen = RawScreen::into_raw_mode().unwrap();
    let input = crossterm::input();
    let term = crossterm::terminal();
    let mut cursor = crossterm::cursor();

    let prompt = "$ ";
    let x = prompt.len();
    let mut y = 0;
    let mut x = prompt.len();
    term.clear(ClearType::All);

    let mut line = String::with_capacity(1024);
    line.push_str(prompt);

    let mut stdin = input.read_sync();
    loop {
        cursor.goto(0, y);
        term.clear(ClearType::CurrentLine);

        for ch in line.chars() {
            term.write(ch);
        }

        if x != line.len() {
            cursor.goto(x as u16, y);
        }

        if let Some(event) = stdin.next() {
           if let InputEvent::Keyboard(key_event) = event {
                match key_event {
                    KeyEvent::Char(ch) => if ch == '\n' {
                        if &line[prompt.len()..] == "exit" {
                            return;
                        }

                        cursor.move_down(1);
                        y += 1;
                        x = 0;

                        line.clear();
                        line.push_str(prompt);
                    } else {
                        line.push(ch);
                        x += 1;
                    },
                    KeyEvent::Backspace => {
                        if x > prompt.len() {
                            x -= 1;
                            line.remove(x);
                        }
                    },
                    KeyEvent::Delete => {
                        if x > prompt.len() && x < line.len() {
                            line.remove(x);
                        }
                    }
                    KeyEvent::Left => {
                        cursor.move_left(1);
                        x -= 1;
                    }
                    KeyEvent::Esc => return,
                    _ => {
                        dbg!(key_event);
                    }
                }
            }
        }
    }
}