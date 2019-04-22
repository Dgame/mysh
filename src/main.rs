use crate::config::Config;
use crate::shell::Shell;
use std::fs::File;
use std::io::Read;

mod cell;
mod config;
mod drawable;
mod line;
mod prompt;
mod shell;
mod widget;

fn load_config() -> std::io::Result<String> {
    let mut file = File::open("mysh.toml")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}

fn main() {
    let config = if let Ok(content) = load_config() {
        toml::from_str(&content).unwrap()
    } else {
        Config::default()
    };

    //    dbg!(config);

    let mut shell = shell::MyShell::new(&config, move |line| Box::new(line::MyLine::new(line)));
    shell.clear();
    shell.run();
}
