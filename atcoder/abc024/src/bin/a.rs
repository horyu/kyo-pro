#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        k: usize,
        s: usize,
        t: usize
    };
    let num = s + t;
    println!(
        "{}",
        if num >= k {
            a * s + b * t - c * num
        } else {
            a * s + b * t
        }
    );
}
