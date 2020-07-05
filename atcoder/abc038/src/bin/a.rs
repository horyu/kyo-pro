#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: Chars
    };
    println!(
        "{}",
        if s.last().unwrap() == &'T' {
            "YES"
        } else {
            "NO"
        }
    );
}
