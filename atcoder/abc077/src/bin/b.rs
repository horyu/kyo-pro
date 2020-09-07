#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize
    };
    let num = (1..std::usize::MAX).find(|i| i * i > n).unwrap() - 1;
    println!("{}", num * num);
}
