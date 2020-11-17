#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut n: usize,
        m: usize,
        mut xx: [isize; m]
    };
    if n >= m || m == 1 {
        println!("0");
        return;
    }
    xx.sort();
    if n == 1 {
        println!("{}", xx[n - 1] - xx[0]);
        return;
    }
    let mut distances: Vec<isize> = xx.windows(2).map(|tmp| tmp[1] - tmp[0]).collect();
    distances.sort();
    for _ in 0..(n - 1) {
        distances.remove(distances.len() - 1);
    }
    println!("{}", distances.iter().sum::<isize>());
}
