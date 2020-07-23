#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut s: String
    };
    println!("{}", s.replacen(',', &" ", 2));
}
