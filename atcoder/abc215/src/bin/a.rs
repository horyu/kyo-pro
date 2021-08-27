use std::io::{self, Read, Write};

fn main() {
    let mut buffer = String::new();
    let mut stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_to_string(&mut buffer).unwrap();
    let rs = if buffer == "Hello,World!\n" {
        "AC"
    } else {
        "WA"
    };
    io::stdout().write_all(rs.as_bytes()).unwrap();
}
