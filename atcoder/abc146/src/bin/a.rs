#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: String
    };
    let arr = ["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"];
    println!("{}", 7 - arr.iter().position(|&a| a == s).unwrap());
}
