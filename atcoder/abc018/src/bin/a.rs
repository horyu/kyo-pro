#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        abc: [usize; 3]
    };
    let mut sorted = abc.clone();
    sorted.sort();
    sorted.reverse();
    for score in abc {
        println!("{}", sorted.iter().position(|&x| x == score).unwrap() + 1);
    }
}
