#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize,
        k: usize
    };
    let mut count = 0;
    let num = (1..=std::cmp::min(a, b))
        .rev()
        .find(|i| {
            count += ((a % i == 0) && (b % i == 0)) as usize;
            count == k
        })
        .unwrap();
    println!("{}", num);
}
