#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    };
    for i in 1..=b {
        if a * i % b == c {
            println!("YES");
            return;
        }
    }
    println!("NO");
}
