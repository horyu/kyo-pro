#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        dd: [usize; n]
    };
    let rs = dd
        .into_iter()
        .combinations(2)
        .fold(0usize, |acc, dd| acc + dd.iter().product::<usize>());
    println!("{}", rs);
}
