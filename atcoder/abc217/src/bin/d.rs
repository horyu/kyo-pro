#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        l: usize,
        q: usize,
        ccxx: [(usize, usize); q]
    };
    let mut hm = std::collections::BTreeMap::new();
    hm.insert(l, l);
    for (c, x) in ccxx {
        if c == 1 {
            let (&k, &v) = hm.range(x..=l).into_iter().next().unwrap();
            hm.insert(x, v - (k - x));
            hm.insert(k, k - x);
        } else {
            let (_, v) = hm.range(x..=l).into_iter().next().unwrap();
            println!("{}", v);
        }
    }
}
