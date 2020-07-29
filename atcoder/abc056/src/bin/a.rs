#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: String,
        b: String
    };
    let arr = ["H", "D", "H"];
    let rs = arr[((a == "H") as usize + (b == "H") as usize)];
    println!("{}", rs);
}
