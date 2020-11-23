#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: String,
    };
    println!(
        "{}",
        ["TBD", "Heisei"][(s.as_str() <= "2019/04/30") as usize]
    );
}
