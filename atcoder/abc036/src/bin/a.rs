#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize
    };

    println!("{}", if b % a == 0 { b / a } else { b / a + 1 });
}
