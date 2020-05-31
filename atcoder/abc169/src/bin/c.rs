#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: u128,
        mut b: String
    };
    b.retain(|c| c != '.');
    println!("{}", a * b.parse::<u128>().unwrap() / 100);
}
