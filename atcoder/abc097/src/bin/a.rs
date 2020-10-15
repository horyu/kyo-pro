#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
        d: isize,
    };
    println!(
        "{}",
        ["No", "Yes"]
            [(((a - c).abs() <= d) || ((a - b).abs() <= d) && (b - c).abs() <= d) as usize]
    );
}
