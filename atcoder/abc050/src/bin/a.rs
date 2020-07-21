#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: isize,
        op: char,
        b: isize
    };
    let rs = match op {
        '+' => a + b,
        '-' => a - b,
        _ => unreachable!(),
    };
    println!("{}", rs);
}
