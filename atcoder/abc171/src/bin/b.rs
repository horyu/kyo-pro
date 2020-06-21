#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        k: usize,
        mut pp: [usize; n]
    };
    pp.sort();
    let sum = pp[0..k].iter().sum::<usize>();
    println!("{}", sum);
}
