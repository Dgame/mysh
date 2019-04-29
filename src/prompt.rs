use crate::config;
use crate::drawable::{Drawable, RenderTarget};
use crate::widget;

pub struct Prompt {
    widgets: Vec<Box<widget::Widget>>,
    config: config::Prompt,
}

impl Prompt {
    pub fn new(config: &config::Prompt) -> Self {
        let mut widgets: Vec<Box<widget::Widget>> = Vec::new();

        let user = if let Some(ref user) = config.user {
            Some(widget::User::new(user))
        } else {
            None
        };

        widgets.push(Box::new(widget::Location { user }));

        let widget = widget::Caret::new(&config.caret);
        widgets.push(Box::new(widget));

        Self {
            widgets,
            config: config.clone(),
        }
    }
}

impl Drawable for Prompt {
    fn render_on(&self, target: &mut RenderTarget) {
        for widget in self.widgets.iter() {
            widget.render_on(target);
        }
    }
}
