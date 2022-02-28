#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n:usize,
        m:usize,
        aabb:[(Usize1,Usize1); m]
    };
    let mut vvv = vec![vec![]; n];
    for (a, b) in aabb.into_iter().sorted() {
        vvv[a].push(b);
    }

    let mut uf = UnionFind::new(n);
    let mut rev_rs = vec![0];
    let mut cnt = 0;
    for i in (1..n).rev() {
        cnt += 1;
        for &v in &vvv[i] {
            if uf.union(i, v) {
                cnt -= 1;
            }
        }
        rev_rs.push(cnt);
    }
    println!("{}", rev_rs.into_iter().rev().join("\n"));
}
