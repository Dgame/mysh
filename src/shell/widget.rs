use crate::drawable::Drawable;
use crate::{config, shell};
use termion::color::{self, Rgb};

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
    fn render_on(&self, term: &mut shell::Terminal) {
        let config::Rgb(r, g, b) = self.config.color;
        let text = whoami::username();

        term.in_color(Some(&Rgb(r, g, b))).write_text(&text);
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
    fn render_on(&self, term: &mut shell::Terminal) {
        use std::env;

        let cur_dir = env::current_dir()
            .map(|dir| dir.as_os_str().to_string_lossy().to_string())
            .unwrap();

        if let Some(ref user) = self.user {
            user.render_on(term);
        }

        term.in_color(None).write_text(" in ");
        term.in_color(Some(&color::LightGreen)).write_text(&cur_dir);
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
    fn render_on(&self, term: &mut shell::Terminal) {
        let config::Rgb(r, g, b) = self.config.color;
        let text = if self.is_admin {
            self.config.admin.to_owned()
        } else {
            self.config.user.to_owned()
        };

        if self.is_on_newline() {
            term.newline()
                .in_color(Some(&Rgb(r, g, b)))
                .write_text(&text);
        } else {
            term.in_color(Some(&Rgb(r, g, b))).write_text(&text);
        }
    }
}
