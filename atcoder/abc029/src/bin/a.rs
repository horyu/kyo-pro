use std::io::{self, Read};
fn main() {
    let mut buffer = Vec::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_end(&mut buffer).unwrap();
    if let Some(last) = buffer.last_mut() {
        *last = b's';
    }
    let s = std::str::from_utf8(&buffer).unwrap();
    println!("{}", s);
}
