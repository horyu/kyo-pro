#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        k: Isize1,
        x: isize,
    };
    let rs = ((x - k)..=(x + k))
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", rs);
}
