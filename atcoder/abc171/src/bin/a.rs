#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: char
    };
    println!("{}", if a.is_lowercase() { "a" } else { "A" });
}
