#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: Chars
    };
    println!(
        "{}",
        ["No", "Yes"][((n[1] == n[2]) && ((n[0] == n[1]) || (n[2] == n[3]))) as usize]
    );
}
