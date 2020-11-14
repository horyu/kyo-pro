#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut hh: [usize; n]
    };
    let mut count = 0;
    for _ in 0..(*hh.iter().max().unwrap()) {
        for group in hh.split_mut(|&h| h == 0) {
            if group.is_empty() {
                continue;
            }
            for h in group {
                *h -= 1;
            }
            count += 1;
        }
    }
    println!("{}", count);
}
