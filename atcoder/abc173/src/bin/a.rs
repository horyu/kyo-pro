#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize
    };
    if n % 1000 == 0 {
        println!("0");
    } else {
        println!("{}", 1000 - n % 1000);
    }
}
