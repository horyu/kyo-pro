#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{BTreeSet, HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        ww: [usize; n]
    };
    let mut bts = BTreeSet::new();
    for w in ww {
        if let Some(&e) = bts.range(w..).next() {
            bts.remove(&e);
        }
        bts.insert(w);
    }
    println!("{}", bts.len());
}
