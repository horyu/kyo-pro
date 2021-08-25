#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        s: usize,
        t: usize,
    };
    let mut cnt = 0;
    for a in 0..=s {
        for b in 0..=(s - a) {
            for c in 0..=(s - a - b) {
                if a * b * c <= t {
                    cnt += 1;
                }
            }
        }
    }
    println!("{}", cnt);
}
