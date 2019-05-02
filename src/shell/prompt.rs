use crate::drawable::Drawable;
use crate::{config, shell};

pub struct Prompt {
    widgets: Vec<Box<shell::Widget>>,
}

impl Prompt {
    pub fn new(config: &config::Prompt) -> Self {
        let mut widgets: Vec<Box<shell::Widget>> = Vec::new();

        let user = if let Some(ref user) = config.user {
            Some(shell::User::new(user))
        } else {
            None
        };

        widgets.push(Box::new(shell::Location { user }));
        let widget = shell::Caret::new(&config.caret);
        widgets.push(Box::new(widget));

        Self { widgets }
    }
}

impl Drawable for Prompt {
    fn render_on(&self, term: &mut shell::Terminal) {
        for widget in self.widgets.iter() {
            widget.render_on(term);
        }
    }
}
