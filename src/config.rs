use serde_derive::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn rgb(&self) -> (u8, u8, u8) {
        (self.red, self.green, self.blue)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            red: 245,
            green: 245,
            blue: 245,
        }
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
    pub color: Color,
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
    pub color: Color,
    #[serde(default)]
    pub on_newline: bool,
}

impl Default for Caret {
    fn default() -> Self {
        Self {
            render: true,
            user: default_user_caret(),
            admin: default_admin_caret(),
            color: Color::default(),
            on_newline: false,
        }
    }
}

fn default_padding() -> u16 {
    1
}

#[derive(Debug, Clone, Deserialize)]
pub struct Line {
    pub capacity: u16,
    #[serde(default = "default_padding")]
    pub left_padding: u16,
    #[serde(default)]
    pub color: Color,
}

impl Default for Line {
    fn default() -> Self {
        Self {
            capacity: 1024,
            left_padding: default_padding(),
            color: Color::default(),
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

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub prompt: Prompt,
    pub line: Line,
}
