#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        x: usize
    };
    let div: usize = x / 11;
    let diff: usize = x % 11;
    if diff == 0 {
        println!("{}", div * 2);
    } else if diff <= 6 {
        println!("{}", div * 2 + 1);
    } else {
        println!("{}", div * 2 + 2);
    }
}
