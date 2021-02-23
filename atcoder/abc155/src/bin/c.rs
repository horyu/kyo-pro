#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        ss: [String; n]
    };
    let mut hm = std::collections::HashMap::new();
    let mut max = 0;
    for s in ss {
        let count = hm.entry(s).or_insert(0);
        *count += 1;
        max = max.max(*count);
    }
    let mut keys = hm
        .iter()
        .filter(|(_k, v)| **v == max)
        .map(|(k, _v)| k)
        .collect_vec();
    keys.sort_unstable();
    for s in keys {
        println!("{}", s);
    }
}
