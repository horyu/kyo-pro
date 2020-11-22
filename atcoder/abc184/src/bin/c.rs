#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        r1: isize,
        c1: isize,
        r2: isize,
        c2: isize,
    };
    if (r1 == r2) && (c1 == c2) {
        println!("0");
        return;
    }
    if (r1 + c1 == r2 + c2) || (r1 - c1 == r2 - c2) || ((r1 - r2).abs() + (c1 - c2).abs() <= 3) {
        println!("1");
        return;
    }
    if (-3isize..=3).any(|k| (r1 + c1 == r2 + c2 + k) || (r1 - c1 == r2 - c2 + k))
        || ((r1 + c1 - r2 - c2) % 2 == 0)
        || ((r1 - r2).abs() + (c1 - c2).abs() <= 6)
    {
        println!("2");
        return;
    }
    println!("3");
}
