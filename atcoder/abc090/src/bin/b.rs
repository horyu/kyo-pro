#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize
    };
    let rs = (a..=b).fold(0, |acc, x| {
        acc + ((((x % 10) == (x / 10000)) && (x / 10 % 10) == (x / 1000 % 10)) as i32)
    });
    println!("{}", rs);
}
