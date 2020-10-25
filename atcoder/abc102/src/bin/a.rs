#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
    };
    println!("{}", if n % 2 == 0 { n } else { 2 * n });
}
