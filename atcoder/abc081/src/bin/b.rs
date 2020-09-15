#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n]
    };
    let mut count = 0;
    while aa.iter().all(|&a| a % 2 == 0) {
        aa = aa.iter().map(|&a| a / 2).collect::<Vec<usize>>();
        count += 1;
    }
    println!("{}", count);
}
