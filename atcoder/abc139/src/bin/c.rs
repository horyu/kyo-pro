#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        hh: [usize; n]
    };
    let mut count = 0;
    let mut max_count = 0;
    let mut pre = 0;
    for h in hh {
        if h <= pre {
            count += 1;
            max_count = max_count.max(count);
        } else {
            count = 0;
        }
        pre = h;
    }
    println!("{}", max_count);
}
