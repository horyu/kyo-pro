#![allow(unused_imports)]
// use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        ccssff: [(usize, usize, usize); n - 1]
    };
    for l in 0..n {
        let mut now = 0;
        for &(c, s, f) in &ccssff[l..] {
            if now <= s {
                now = s + c;
            } else {
                now = f * now.div_ceil(&f) + c;
            }
        }
        println!("{}", now);
    }
}
