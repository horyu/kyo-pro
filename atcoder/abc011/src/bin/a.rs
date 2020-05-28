#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: isize
    };
    println!("{}", (n % 12) + 1);
}
