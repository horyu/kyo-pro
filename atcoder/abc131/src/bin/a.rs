#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut s: Chars
    };
    s.dedup();
    println!("{}", ["Good", "Bad"][(s.len() != 4) as usize]);
}
