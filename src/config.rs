use serde_derive::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Default for Rgb {
    fn default() -> Self {
        Self(245, 245, 245)
    }
}

fn render_always() -> bool {
    true
}

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    #[serde(default = "render_always")]
    pub render: bool,
    #[serde(default)]
    pub color: Rgb,
}

fn default_user_caret() -> String {
    String::from("$ ")
}

fn default_admin_caret() -> String {
    String::from("# ")
}

#[derive(Debug, Clone, Deserialize)]
pub struct Caret {
    #[serde(default = "render_always")]
    pub render: bool,
    #[serde(default = "default_user_caret")]
    pub user: String,
    #[serde(default = "default_admin_caret")]
    pub admin: String,
    #[serde(default)]
    pub color: Rgb,
    #[serde(default)]
    pub on_newline: bool,
}

impl Default for Caret {
    fn default() -> Self {
        Self {
            render: true,
            user: default_user_caret(),
            admin: default_admin_caret(),
            color: Rgb::default(),
            on_newline: false,
        }
    }
}

fn default_padding() -> u8 {
    1
}

#[derive(Debug, Clone, Deserialize)]
pub struct Line {
    pub capacity: u16,
    #[serde(default = "default_padding")]
    pub left_padding: u8,
    #[serde(default)]
    pub color: Rgb,
}

impl Default for Line {
    fn default() -> Self {
        Self {
            capacity: 1024,
            left_padding: default_padding(),
            color: Rgb::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Prompt {
    pub user: Option<User>,
    #[serde(default)]
    pub caret: Caret,
}

impl Default for Prompt {
    fn default() -> Self {
        Self {
            user: None,
            caret: Caret::default(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Colorize {
    pub command: Option<Rgb>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct History {
    pub filename: String,
    pub size: usize,
}

impl Default for History {
    fn default() -> Self {
        Self {
            size: 10_000,
            filename: String::from("mysh-history.log"),
        }
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub prompt: Prompt,
    #[serde(default)]
    pub line: Line,
    #[serde(default)]
    pub colorize: Colorize,
    #[serde(default)]
    pub history: History,
}
