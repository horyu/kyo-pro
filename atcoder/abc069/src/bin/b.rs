#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: Chars
    };
    println!("{}{}{}", s[0], s.len() - 2, s.last().unwrap());
}
