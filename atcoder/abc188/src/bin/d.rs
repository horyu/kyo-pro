#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        c_all: isize,
        aabbcc: [(isize, isize, isize); n]
    };
    use std::collections::BTreeMap;
    let mut hm: BTreeMap<isize, isize> = BTreeMap::new();
    for (a, b, c) in aabbcc {
        *hm.entry(a).or_insert(0) += c;
        *hm.entry(b + 1).or_insert(0) -= c;
    }
    let mut sum = 0;
    let mut current_c = 0isize;
    for ((l, lc), (r, _rc)) in hm.iter().tuple_windows() {
        current_c += lc;
        sum += (r - l) * c_all.min(current_c);
    }
    println!("{}", sum);
}
