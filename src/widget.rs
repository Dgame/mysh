use crate::config;
use crate::drawable::{Drawable, RenderTarget};
use std::io::Write;
use termion::color::{self, Fg, Reset, Rgb};

pub trait Widget: Drawable {
    fn should_render(&self) -> bool;
}

pub struct User {
    pub config: config::User,
}

impl User {
    pub fn new(config: &config::User) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

impl Widget for User {
    fn should_render(&self) -> bool {
        self.config.render
    }
}

impl Drawable for User {
    fn render_on(&self, target: &mut RenderTarget) {
        let (r, g, b) = self.config.color.rgb();
        let text = whoami::username();

        target.cursor.x += text.len() as u16;

        write!(
            target.terminal,
            "{reset}{color}{text}",
            reset = Fg(Reset),
            color = Fg(Rgb(r, g, b)),
            text = text
        )
        .unwrap();
    }
}

pub struct Location {
    pub user: Option<User>,
}

impl Widget for Location {
    fn should_render(&self) -> bool {
        true
    }
}

impl Drawable for Location {
    fn render_on(&self, target: &mut RenderTarget) {
        use std::env;

        let cur_dir = env::current_dir()
            .map(|dir| dir.as_os_str().to_string_lossy().to_string())
            .unwrap();
        if let Some(ref user) = self.user {
            user.render_on(target);
        }

        write!(
            target.terminal,
            "{reset} in {color}{dir}",
            reset = Fg(Reset),
            color = Fg(color::LightGreen),
            dir = cur_dir
        )
        .unwrap();
    }
}

pub struct Caret {
    pub config: config::Caret,
    pub is_admin: bool,
}

impl Caret {
    pub fn new(config: &config::Caret) -> Self {
        Self {
            config: config.clone(),
            is_admin: false,
        }
    }

    pub fn is_on_newline(&self) -> bool {
        self.config.on_newline
    }
}

impl Widget for Caret {
    fn should_render(&self) -> bool {
        self.config.render
    }
}

impl Drawable for Caret {
    fn render_on(&self, target: &mut RenderTarget) {
        use termion::cursor;

        let (r, g, b) = self.config.color.rgb();
        let text = if self.is_admin {
            self.config.admin.to_owned()
        } else {
            self.config.user.to_owned()
        };

        target.cursor.x = text.len() as u16 + 1;
        if self.is_on_newline() {
            target.cursor.y += 1;

            write!(
                target.terminal,
                "{}{color}{text}",
                cursor::Goto(1, target.cursor.y),
                color = Fg(Rgb(r, g, b)),
                text = text
            )
            .unwrap();
        } else {
            write!(
                target.terminal,
                "{color}{text}",
                color = Fg(Rgb(r, g, b)),
                text = text
            )
            .unwrap();
        }
    }
}
