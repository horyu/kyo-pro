#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
        bb: [usize; n],
        cc: [Usize1; n],
    };
    let mut a_counts = HashMap::new();
    for a in aa {
        *a_counts.entry(a).or_insert(0) += 1usize;
    }
    let mut b_counts = HashMap::new();
    for c in cc {
        *b_counts.entry(bb[c]).or_insert(0) += 1usize;
    }
    let (small, big) = if a_counts.len() < b_counts.len() {
        (a_counts, b_counts)
    } else {
        (b_counts, a_counts)
    };
    let rs = small
        .into_iter()
        .map(|(x, x_count)| {
            if let Some(y_count) = big.get(&x) {
                x_count * y_count
            } else {
                0
            }
        })
        .sum::<usize>();
    println!("{}", rs);
}
