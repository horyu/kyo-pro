#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        c: char
    };
    println!("{}", ((c as u8) + 1) as char);
}
