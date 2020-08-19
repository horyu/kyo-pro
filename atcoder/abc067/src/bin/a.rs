#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: u8,
        b: u8
    };
    println!(
        "{}",
        ["Impossible", "Possible"][((a % 3 == 0) || (b % 3 == 0) || ((a + b) % 3 == 0)) as usize]
    );
}
