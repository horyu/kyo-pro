#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: String,
        b: String
    };
    println!("{}", if a.len() > b.len() { a } else { b });
}
