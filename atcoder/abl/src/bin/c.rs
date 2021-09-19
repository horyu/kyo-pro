#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        m: usize,
        aabb: [(Usize1, Usize1); m]
    };
    let mut uf = UnionFind::new(n);
    for (a, b) in aabb {
        uf.union(a, b);
    }
    let mut labels = uf.into_labeling();
    labels.sort_unstable();
    labels.dedup();
    println!("{}", labels.len() - 1);
}
