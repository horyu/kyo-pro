#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: isize,
        b: isize,
    };
    println!("{}", if (a < 10) && (b < 10) { a * b } else { -1 });
}
