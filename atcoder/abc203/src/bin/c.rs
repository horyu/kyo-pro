#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        mut k: usize,
        mut aabb: [(usize, usize); n]
    };
    aabb.sort_unstable_by_key(|ab| ab.0);
    let mut aabb: std::collections::VecDeque<_> = aabb.into_iter().collect();
    let mut pos = 0usize;
    while k > 0 {
        pos += k;
        k = 0;
        while let Some(&(a, b)) = aabb.get(0) {
            if a <= pos {
                k += b;
                aabb.pop_front();
            } else {
                break;
            }
        }
    }
    println!("{}", pos);
}
