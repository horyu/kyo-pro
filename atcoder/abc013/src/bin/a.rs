#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        x: char
    };
    println!("{}", (x as u8) + 1 - b'A');
}
