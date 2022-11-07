#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        r: usize,
        c: usize,
        k: usize,
        sss: [Chars; r],
    };
    if [r, c].iter().any(|&x| x < 2 * k - 1) {
        println!("0");
        return;
    }
    let mut s = sss
        .into_iter()
        .map(|s| s.into_iter().map(|s| s == 'o').collect_vec())
        .collect_vec();
    for kk in 1..k {
        let mut new_s = vec![vec![false; c]; r];
        for i in kk..(r - kk) {
            for j in kk..(c - kk) {
                if s[i - 1][j] && s[i][j - 1] && s[i][j] && s[i][j + 1] && s[i + 1][j] {
                    new_s[i][j] = true;
                }
            }
        }
        s = new_s;
    }
    let rs = s
        .into_iter()
        .fold(0, |acc, s| acc + s.into_iter().filter(|&tf| tf).count());
    println!("{rs}");
}
