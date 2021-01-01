#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        s: String
    };
    if a >= 3200 {
        println!("{}", s);
    } else {
        println!("red");
    }
}
