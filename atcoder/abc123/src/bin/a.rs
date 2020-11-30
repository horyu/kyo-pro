#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        xx: [isize; 5],
        k: isize,
    };
    let tf = xx
        .into_iter()
        .combinations(2)
        .all(|x| (x[0] - x[1]).abs() <= k);
    println!("{}", [":(", "Yay!"][tf as usize]);
}
