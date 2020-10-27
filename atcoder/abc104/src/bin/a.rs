#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        r: usize
    };
    println!(
        "{}",
        if r < 1200 {
            "ABC"
        } else if r < 2800 {
            "ARC"
        } else {
            "AGC"
        }
    );
}
