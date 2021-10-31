#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        aabb: [(Usize1, Usize1); n - 1]
    };
    let mut hm = HashMap::new();
    for (a, b) in aabb {
        *hm.entry(a).or_insert(0usize) += 1;
        *hm.entry(b).or_insert(0) += 1;
    }
    let tf = hm.len() == n && hm.values().any(|&cnt| cnt == n - 1);
    println!("{}", ["No", "Yes"][tf as usize]);
}
