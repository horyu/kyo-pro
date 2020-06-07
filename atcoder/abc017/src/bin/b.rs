#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut x: String
    };
    for from in ["ch", "o", "k", "u"].iter() {
        x = x.replace(from, "");
    }
    println!("{}", if x.is_empty() { "YES" } else { "NO" });
}
