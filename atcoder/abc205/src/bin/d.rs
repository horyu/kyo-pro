#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
fn main() {
    input! {
        n: usize,
        q: usize,
        aa: [usize; n],
        kk: [usize; q]
    };
    // aaの各値が何番前の整数か
    let bb = aa.iter().enumerate().map(|(i, &a)| a - i - 1).collect_vec();
    // eprintln!("{:?}", bb);
    for k in kk {
        // eprintln!("{} {} {}", k, bb.lower_bound(&k), bb.upper_bound(&k));
        let lower_bound = bb.lower_bound(&k);
        if lower_bound == 0 {
            println!("{}", k);
        } else if lower_bound >= n {
            println!("{}", k + n);
        } else {
            println!("{}", aa[lower_bound - 1] + k - bb[lower_bound - 1]);
        }
    }
}

// https://github.com/hatoo/competitive-rust-snippets/blob/master/src/binary_search.rs
/// Equivalent to std::lowerbound and std::upperbound in c++
pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}
