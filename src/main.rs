mod shell;

fn main() {
    use crate::shell::MyShell;

    let mut shell = MyShell::new();
    shell.execute();
}
