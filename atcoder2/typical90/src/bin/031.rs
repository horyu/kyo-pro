#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        ww: [usize; n],
        bb: [usize; n],
    };
    const MAX: usize = 1500;
    let mut grundy = vec![vec![0; MAX]; 50 + 1];
    for i in 0..=50 {
        for j in 0..MAX {
            let mut mex = vec![false; MAX];
            if 0 < i && j + i < MAX {
                mex[grundy[i - 1][j + i]] = true;
            }
            for k in 1..=(j / 2) {
                mex[grundy[i][j - k]] = true;
            }
            if let Some(k) = mex.iter().position(|&m| !m) {
                grundy[i][j] = k;
            }
        }
    }

    let sum_xor = izip!(ww, bb)
        .map(|(w, b)| grundy[w][b])
        .fold(0, |acc, g| acc ^ g);
    let rs = ["First", "Second"][(sum_xor == 0) as usize];
    println!("{rs}");
}
