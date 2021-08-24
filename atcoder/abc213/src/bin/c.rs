#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        _h: usize,
        _w: usize,
        n: usize,
    };
    let mut aabb = vec![];
    let mut hh = HashSet::new();
    let mut ww = HashSet::new();
    for _ in 0..n {
        input! {a: Usize1, b: Usize1};
        aabb.push((a, b));
        hh.insert(a);
        ww.insert(b);
    }
    let hh = to_hm(hh);
    let ww = to_hm(ww);
    for (a, b) in aabb {
        println!("{} {}", hh.get(&a).unwrap(), ww.get(&b).unwrap());
    }
}

fn to_hm(hs: HashSet<usize>) -> HashMap<usize, usize> {
    let mut hm = HashMap::new();
    for (i, v) in hs.into_iter().sorted().enumerate() {
        hm.insert(v, i + 1);
    }
    hm
}
