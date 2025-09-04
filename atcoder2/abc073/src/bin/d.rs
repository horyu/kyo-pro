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
        m: usize,
        r: usize,
        rr: [Usize1; r],
        aabbcc: [(Usize1, Usize1, usize); m],
    };
    let mut vvv = vec![vec![std::usize::MAX >> 2; n]; n];
    for i in 0..n {
        vvv[i][i] = 0;
    }
    for (a, b, c) in aabbcc {
        vvv[a][b] = c;
        vvv[b][a] = c;
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                vvv[i][j] = vvv[i][j].min(vvv[i][k] + vvv[k][j]);
            }
        }
    }
    let mut rs = std::usize::MAX;
    for ii in (0..r).permutations(r) {
        rs = rs.min(
            ii.into_iter()
                .tuple_windows()
                .fold(0, |acc, (i, j)| acc + vvv[rr[i]][rr[j]]),
        );
    }
    println!("{rs}");
}
