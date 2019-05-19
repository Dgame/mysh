#![allow(dead_code)]

use std::fs::File;
use std::io::Read;

use crate::behaviour::{ExecutableWordColorizer, WordColorizeBehaviour};
use crate::config::Config;
use crate::shell::Shell;

mod behaviour;
mod config;
mod drawable;
mod history;
mod my;
mod path;
mod pool;
mod shell;

fn load_config() -> std::io::Result<String> {
    let mut file = File::open("mysh.toml")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}

fn main() {
    use flexi_logger::{Duplicate, Logger};

    Logger::with_str("info, mysh = debug")
        .log_to_file()
        .duplicate_to_stderr(Duplicate::Warn)
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));

    let config = if let Ok(content) = load_config() {
        toml::from_str(&content).unwrap()
    } else {
        Config::default()
    };

    let mut colorizer = WordColorizeBehaviour::new(&config.colorize);
    colorizer.add_colorizer(Box::new(ExecutableWordColorizer::new()));

    let mut line = my::Line::new(&config.line);
    line.add_behaviour(Box::new(colorizer));

    let mut shell = my::Shell::new(&config, Box::new(my::Terminal::new()), Box::new(line));
    shell.clear();
    shell.run();
}
