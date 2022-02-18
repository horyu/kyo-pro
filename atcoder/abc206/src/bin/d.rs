#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let mut rs = 0;
    let mut uf = UnionFind::new(1e5 as usize * 2 + 2);
    let mut l = 0;
    let mut r = n - 1;
    while l < r {
        if uf.union(aa[l], aa[r]) {
            rs += 1;
        }
        l += 1;
        r -= 1;
    }
    println!("{rs}");
}
