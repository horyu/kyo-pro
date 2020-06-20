#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize
    };
    println!("{}", (a / 2) * (a - a / 2));
}
