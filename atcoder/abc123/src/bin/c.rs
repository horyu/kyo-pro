#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        xx: [usize; 5]
        // a: usize,
        // b: usize,
        // c: usize,
        // d: usize,
        // e: usize,
    };
    let min = xx.iter().min().unwrap();
    println!("{}", n / min + ((n % min != 0) as usize) + 4);
}
