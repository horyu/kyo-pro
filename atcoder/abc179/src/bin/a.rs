#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: String
    };
    println!("{}{}", s, ["s", "es"][(s.ends_with('s')) as usize]);
}
