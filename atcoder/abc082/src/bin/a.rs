#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: f64,
        b: f64
    };
    println!("{}", ((a + b) / 2.0).ceil() as i32);
}
