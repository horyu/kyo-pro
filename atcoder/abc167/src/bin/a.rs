#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        s: String,
        t: String
    };
    println!("{}", if t.starts_with(&s) { "Yes" } else { "No" });
}
