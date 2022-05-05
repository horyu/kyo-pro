#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        k: usize,
        aa: [usize; n]
    };
    let mut lbtm = BTreeMap::new();
    let mut rbtm = BTreeMap::new();
    for (i, &a) in aa.iter().enumerate().take(k).rev() {
        lbtm.entry(a).or_insert(i);
    }
    for (j, &a) in aa.iter().enumerate().skip(k) {
        if rbtm.range(a..).next().is_none() {
            rbtm.insert(a, j);
        }
    }
    let mut rs = std::usize::MAX;
    for (a, i) in lbtm {
        if let Some((_, j)) = rbtm.range((a + 1)..).next() {
            rs = rs.min(j - i);
        }
    }
    if rs == std::usize::MAX {
        println!("-1");
    } else {
        println!("{rs}");
    }
}
