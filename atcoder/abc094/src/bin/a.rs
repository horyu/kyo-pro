#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize,
        x: usize,
    };
    println!("{}", ["NO", "YES"][((a <= x) && (x < a + b)) as usize]);
}
