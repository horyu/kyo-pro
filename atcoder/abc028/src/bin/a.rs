#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize
    };
    println!(
        "{}",
        match n {
            _ if n <= 59 => "Bad",
            _ if n <= 89 => "Good",
            _ if n <= 99 => "Great",
            _ => "Perfect",
        }
    );
}
