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
        uuvv: [(Usize1, Usize1); m]
    };
    let mut uf = UnionFind::new(n);
    let mut hs = HashSet::new();
    for (u, v) in uuvv {
        // 繋がっていたら木ではない　親をたどれるように保存
        if !uf.union(u, v) {
            hs.insert(u);
        }
    }
    // 木ではないグループの親をまとめる
    hs = hs.into_iter().map(|x| uf.find(x)).collect();
    let rs = uf
        .into_labeling()
        .into_iter()
        .enumerate()
        .filter(|(i, x)| i == x && !hs.contains(x))
        .count();
    println!("{rs}");
}
