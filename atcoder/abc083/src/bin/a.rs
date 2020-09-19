#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize
    };
    use std::cmp::Ordering::*;
    println!(
        "{}",
        match (a + b).cmp(&(c + d)) {
            Less => "Right",
            Equal => "Balanced",
            Greater => "Left",
        }
    );
}
