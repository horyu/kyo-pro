#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        xuxu: [(f64, String); n]
    };
    let mut sum = 0f64;
    for (x, u) in xuxu {
        if u == "JPY" {
            sum += x;
        } else {
            sum += x * 380000.0;
        }
    }
    println!("{}", sum);
}
