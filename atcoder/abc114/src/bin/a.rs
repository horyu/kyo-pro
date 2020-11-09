#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        x: usize
    };
    println!("{}", ["NO", "YES"][[7, 5, 3].contains(&x) as usize]);
}
