#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut n: Chars
    };
    n.dedup();
    println!("{}", if n.len() == 1 { "SAME" } else { "DIFFERENT" });
}
