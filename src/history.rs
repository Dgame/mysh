use crate::config;

pub struct History {
    config: config::History,
    offset: usize,
    lines: Vec<String>,
}

impl History {
    pub fn load(config: &config::History) -> Self {
        use std::fs;

        let lines: Vec<String> = fs::read_to_string(&config.filename)
            .and_then(|content| {
                Ok(content
                    .lines()
                    .map(std::borrow::ToOwned::to_owned)
                    .collect())
            })
            .unwrap_or_else(|_| Vec::new());

        Self {
            config: config.clone(),
            offset: lines.len() - 1,
            lines,
        }
    }

    pub fn save(&mut self) {
        use std::fs;

        fs::write(&self.config.filename, self.lines.join("\n").as_bytes())
            .expect("Could not write History")
    }

    pub fn get_previous(&mut self) -> Option<&String> {
        let offset = self.offset;
        if self.offset > 0 {
            self.offset -= 1;
        } else {
            self.reset_offset();
        }

        self.lines.get(offset)
    }

    pub fn reset_offset(&mut self) {
        self.offset = self.lines.len() - 1;
    }

    pub fn search(&self, input: &str) -> Vec<&String> {
        self.lines
            .iter()
            .filter(|line| line.starts_with(input))
            .collect()
    }

    pub fn insert(&mut self, input: String) {
        if let Some(index) = self.lines.iter().position(|item| item == &input) {
            self.lines.remove(index);
        }

        self.lines.push(input);
        self.offset = self.lines.len() - 1;
    }
}

impl Drop for History {
    fn drop(&mut self) {
        self.save()
    }
}
