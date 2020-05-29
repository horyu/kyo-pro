#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize
    };
    println!("{:02}:{:02}:{:02}", n / 3600, n % 3600 / 60, n % 60);
}
