#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::Ordering;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize
    };
    // (b / a) > (d / c)
    // bd > ad
    println!(
        "{}",
        match (b * c).cmp(&(a * d)) {
            Ordering::Greater => "TAKAHASHI",
            Ordering::Equal => "DRAW",
            Ordering::Less => "AOKI",
        }
    );
}
