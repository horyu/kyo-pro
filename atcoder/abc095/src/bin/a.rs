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
            .fold(700, |acc, &c| acc + [0, 100][(c == 'o') as usize])
    );
}
