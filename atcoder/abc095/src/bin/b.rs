#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        x: usize,
        mm: [usize; n]
    };
    let sum = mm.iter().sum::<usize>();
    let min = mm.iter().min().unwrap();
    println!("{}", n + (x - sum) / min);
}
