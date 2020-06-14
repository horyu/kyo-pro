#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        x: usize,
        y: usize
    };
    for i in 0..=x {
        if y == (2 * i + 4 * (x - i)) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
