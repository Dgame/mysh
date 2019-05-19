use crate::config;

pub struct History {
    config: config::History,
    input: Vec<String>,
}

impl History {
    pub fn load(config: &config::History) -> Self {
        use std::fs;

        Self {
            config: config.clone(),
            input: fs::read_to_string(&config.filename)
                .and_then(|content| {
                    Ok(content
                        .lines()
                        .map(std::borrow::ToOwned::to_owned)
                        .collect())
                })
                .unwrap_or_else(|_| Vec::new()),
        }
    }

    pub fn save(&mut self) {
        use std::fs;

        fs::write(&self.config.filename, self.input.join("\n").as_bytes())
            .expect("Could not write History")
    }

    pub fn search(&self, input: &str) -> Vec<&String> {
        self.input
            .iter()
            .filter(|line| line.starts_with(input))
            .collect()
    }

    pub fn insert(&mut self, input: String) {
        self.input.push(input)
    }
}

impl Drop for History {
    fn drop(&mut self) {
        self.save()
    }
}
