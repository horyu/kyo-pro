#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        c: char
    };
    let v = vec!['a', 'i', 'u', 'e', 'o'];
    println!("{}", if v.contains(&c) { "vowel" } else { "consonant" });
}
