#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: String
    };
    let count = (&s)
        .split('+')
        .fold(0, |acc, x| acc + !x.contains('0') as i32);
    println!("{}", count);
}
