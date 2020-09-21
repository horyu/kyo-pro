#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut x: usize,
        y: usize
    };
    let mut count = 0;
    while x <= y {
        count += 1;
        x *= 2;
    }
    println!("{}", count);
}
