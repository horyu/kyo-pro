#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {q: usize};
    let mut btm = BTreeMap::new();
    for _ in 0..q {
        input! {c: usize, x: usize};
        if c == 1 {
            *btm.entry(x).or_insert(0) += 1;
            continue;
        }
        input! {k: usize};
        let mut exists = false;
        if c == 2 {
            let mut count_sum = 0;
            for (num, count) in btm.range(0..=x).rev() {
                count_sum += count;
                if count_sum >= k {
                    println!("{num}");
                    exists = true;
                    break;
                }
            }
        } else {
            let mut count_sum = 0;
            for (num, count) in btm.range(x..std::usize::MAX) {
                count_sum += count;
                if count_sum >= k {
                    println!("{num}");
                    exists = true;
                    break;
                }
            }
        }
        if !exists {
            println!("-1");
        }
    }
}
