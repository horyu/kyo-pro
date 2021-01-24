#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut s: Chars
    };
    s.dedup();
    println!("{}", ["Lost", "Won"][(s.len() == 1) as usize]);
}
