#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        sss: [Chars; n],
    };
    let s = sss
        .into_iter()
        .map(|ss| ss.into_iter().map(|s| s == '#').collect_vec())
        .collect_vec();
    for i in 0..(n - 8) {
        for j in 0..(m - 8) {
            if (0..3).all(|d| {
                s[i][j + d]
                    && s[i + 1][j + d]
                    && s[i + 2][j + d]
                    && !s[i + 3][j + d]
                    && !s[i + 5][j + 6 + d]
                    && s[i + 6][j + 6 + d]
                    && s[i + 7][j + 6 + d]
                    && s[i + 8][j + 6 + d]
            }) && (0..4).all(|d| !s[i + d][j + 3] && !s[i + 5 + d][j + 5])
            {
                println!("{} {}", i + 1, j + 1);
            }
        }
    }
    // println!("{rs}");
}
