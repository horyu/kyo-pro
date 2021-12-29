#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut xx: [isize; m]
    };
    if n >= m {
        println!("0");
        return;
    }
    xx.sort_unstable();
    if n == 1 {
        println!("{}", xx[m - 1] - xx[0]);
        return;
    }
    let mut distances: Vec<isize> = xx.windows(2).map(|tmp| tmp[1] - tmp[0]).collect();
    distances.sort_unstable();
    for _ in 0..(n - 1) {
        distances.pop();
    }
    println!("{}", distances.iter().sum::<isize>());
}
