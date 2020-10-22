#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: Chars
    };
    println!(
        "{}",
        s.iter()
            .fold(0, |acc, &c| acc + [-1, 1][(c == '+') as usize])
    );
}
