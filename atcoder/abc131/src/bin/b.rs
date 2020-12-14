#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: isize,
        l: isize
    };
    let r = l + n - 1;
    let eat = if r <= 0 {
        r
    } else if l >= 0 {
        l
    } else {
        0
    };
    let rs: isize = (r + l) * (r - l + 1) / 2 - eat;
    println!("{}", rs);
}
