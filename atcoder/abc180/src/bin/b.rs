#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        xx: [isize; n]
    };
    println!("{}", xx.iter().fold(0, |acc, x| acc + x.abs()));
    println!(
        "{}",
        xx.iter().fold(0f64, |acc, x| acc + (x * x) as f64).sqrt()
    );
    println!("{}", xx.iter().map(|x| x.abs()).max().unwrap());
}
