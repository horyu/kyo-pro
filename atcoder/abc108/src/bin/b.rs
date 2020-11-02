#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        x1: isize,
        y1: isize,
        x2: isize,
        y2: isize,
    };
    let (x3, y3) = (x2 + y1 - y2, y2 - x1 + x2);
    let (x4, y4) = (x3 + x1 - x2, y3 + y1 - y2);
    println!("{} {} {} {}", x3, y3, x4, y4);
}
