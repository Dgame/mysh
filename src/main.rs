use crate::shell::Shell;

mod shell;

fn main() {
    let mut shell = shell::MyShell::new(move || Box::new(shell::MyLine::new(120)));
    shell.clear();
    shell.run();
}
