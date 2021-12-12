#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        ss: [String; n]
    };
    let mut hm = HashMap::new();
    for s in ss {
        *hm.entry(s).or_insert(0) += 1;
    }
    let e = hm.iter().max_by_key(|kv| kv.1).unwrap();
    println!("{}", e.0);
}
