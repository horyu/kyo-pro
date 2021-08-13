#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let mut hm = HashMap::new();
    for a in aa {
        *hm.entry(a).or_insert(0usize) += 1;
    }
    let rs = n * (n - 1) / 2
        - hm.values()
            .into_iter()
            .filter(|cnt| **cnt > 1)
            .map(|cnt| cnt * (cnt - 1) / 2)
            .sum::<usize>();
    println!("{}", rs);
}
