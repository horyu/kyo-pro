#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: Chars
    };
    let l = s.iter().position(|&c| c == 'A').unwrap();
    let r = s.iter().rposition(|&c| c == 'Z').unwrap();
    println!("{}", 1 + r - l);
}
