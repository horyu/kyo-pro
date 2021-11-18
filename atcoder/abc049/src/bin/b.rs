#![allow(unused_imports)]
use std::io::{self, prelude::*};

fn main() {
    let mut buf = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut buf).unwrap();

    let bytes = buf.as_bytes();
    let mut iter = bytes.iter().enumerate();

    let space_index = loop {
        if let Some((i, b' ')) = iter.next() {
            break i;
        }
    };
    let lf_index = loop {
        if let Some((i, b'\n')) = iter.next() {
            break i;
        }
    };

    let h: usize = (&buf)[0..(space_index)].parse().unwrap();
    let line_width: usize = (&buf)[(space_index + 1)..(lf_index)]
        .parse::<usize>()
        .unwrap()
        + 1;
    let mut writer = io::BufWriter::new(io::stdout());
    let mut left_index = lf_index + 1;
    for _ in 0..h {
        let slice = &bytes[left_index..(left_index + line_width)];
        writer.write_all(slice).unwrap();
        writer.write_all(slice).unwrap();
        left_index += line_width;
    }
}
