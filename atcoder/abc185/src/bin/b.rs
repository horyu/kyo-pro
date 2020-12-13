#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut n: usize,
        m: usize,
        t: usize,
        abab: [(usize, usize); m]
    };
    let max_n = n;
    let mut pre_t = 0;
    for (a, b) in abab {
        if n <= a - pre_t {
            println!("No");
            return;
        }
        n -= a - pre_t;
        n += b - a;
        n = std::cmp::min(max_n, n);
        pre_t = b;
    }
    if n <= t - pre_t {
        println!("No");
    } else {
        println!("Yes");
    }
}
