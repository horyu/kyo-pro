#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut h: usize
    };
    let mut num = 1usize;
    let mut rs = 0usize;
    while h >= 1 {
        rs += num;
        num *= 2;
        h /= 2;
    }
    println!("{}", rs);
}
