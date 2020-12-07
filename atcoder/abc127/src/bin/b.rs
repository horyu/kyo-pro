#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        r: isize,
        d: isize,
        mut x: isize
    };
    for _ in 0..10 {
        x = r * x - d;
        println!("{}", x);
    }
}
