#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use ac_library_rs::ModInt1000000007;
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        h: usize,
        w: usize,
        aaa: [[usize; w]; h],
    };
    let mut btm = BTreeMap::new();
    for (i, aa) in aaa.iter().enumerate() {
        for (j, a) in aa.iter().enumerate() {
            btm.entry(*a).or_insert_with(Vec::new).push((i, j));
        }
    }
    let mut vvv = vec![vec![ModInt1000000007::new(0); w]; h];
    for (&k, vv) in btm.iter().rev() {
        for &(i, j) in vv {
            let mut v = ModInt1000000007::new(1);
            if 0 < i && k < aaa[i - 1][j] {
                v += vvv[i - 1][j];
            }
            if i < h - 1 && k < aaa[i + 1][j] {
                v += vvv[i + 1][j];
            }
            if 0 < j && k < aaa[i][j - 1] {
                v += vvv[i][j - 1];
            }
            if j < w - 1 && k < aaa[i][j + 1] {
                v += vvv[i][j + 1];
            }
            vvv[i][j] = v;
        }
    }
    // for vv in &vvv {
    //     eprintln!("{}", vv.iter().join(" "));
    // }
    let rs = vvv.into_iter().flatten().sum::<ModInt1000000007>();
    println!("{rs}");
}
