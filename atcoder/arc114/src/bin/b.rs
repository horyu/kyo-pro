#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        ff: [Usize1; n]
    };
    let mut uf = petgraph::unionfind::UnionFind::new(n);
    for (i, f) in ff.into_iter().enumerate() {
        uf.union(i, f);
    }
    let hs = uf.into_labeling().into_iter().collect::<HashSet<_>>();
    let mut rs = 1;
    for _ in 0..hs.len() {
        rs = rs * 2 % 998244353;
    }
    rs = (rs + 998244353 - 1) % 998244353;
    println!("{rs}");
}
