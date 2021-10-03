#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        aabb: [(usize, usize); n]
    };
    let mut btm = std::collections::BTreeMap::new();
    for (a, b) in aabb {
        *btm.entry(a).or_insert(0isize) += 1;
        *btm.entry(a + b).or_insert(0isize) -= 1;
    }
    let mut rs = vec![0usize; n + 1];
    let mut current = 0isize;
    let mut pre_day = 0;
    for (today, num) in btm {
        rs[current as usize] += today - pre_day;
        current += num;
        pre_day = today;
    }
    println!("{}", rs[1..].iter().join(" "));
}
