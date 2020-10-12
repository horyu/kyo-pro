#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
        x: isize,
        y: isize,
    };
    use std::cmp::{max, min};
    let mut v = vec![];
    // 全部AB
    v.push(2 * c * max(x, y));
    // AとBのみ
    v.push(a * x + b * y);
    // (x < y) AをABで残りB
    if x < y {
        v.push(2 * c * x + b * (y - x));
    }
    // (x > y) BをABで残りA
    if x > y {
        v.push(2 * c * y + a * (x - y));
    }
    println!("{}", v.iter().min().unwrap());
}
