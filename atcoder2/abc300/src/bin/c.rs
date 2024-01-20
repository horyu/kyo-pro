#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
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
        h: usize,
        w: usize,
        ccc: [Chars; h],
    };
    let ccc = ccc
        .into_iter()
        .map(|cc| cc.into_iter().map(|c| c == '#').collect_vec())
        .collect_vec();
    let n = h.min(w);
    let mut rs = vec![0; n];
    for i in 1..(h - 1) {
        for j in 1..(w - 1) {
            if ccc[i][j] {
                let mut k = 1;
                while (0..=i.min(j)).contains(&k)
                    && (i + k) < h
                    && (j + k) < w
                    && ccc[i - k][j - k]
                    && ccc[i - k][j + k]
                    && ccc[i + k][j - k]
                    && ccc[i + k][j + k]
                {
                    k += 1;
                }
                if 1 < k {
                    rs[k - 2] += 1;
                }
            }
        }
    }
    let rs = rs.iter().join(" ");
    println!("{rs}");
}
