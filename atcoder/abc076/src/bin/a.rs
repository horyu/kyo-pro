#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        r: isize,
        g: isize
    };
    println!("{}", 2 * g - r);
}
