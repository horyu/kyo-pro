#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize
    };
    println!("{}", if a % 2 == 0 { a - 1 } else { a + 1 });
}
