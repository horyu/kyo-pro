#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        c1: Chars,
        c2: Chars,
        c3: Chars,
    };
    println!("{}{}{}", c1[0], c2[1], c3[2]);
}
