#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [isize; n],
        bb: [isize; n],
    };
    let rs = aa
        .into_iter()
        .zip(bb.into_iter())
        .fold(0, |acc, (a, b)| acc + a * b);
    println!("{}", ["No", "Yes"][(rs == 0) as usize]);
}
