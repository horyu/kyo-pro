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
        uuvv: [(Usize1, Usize1); m],
    };
    let mut vvv = vec![vec![false; n]; n];
    for (u, v) in uuvv {
        vvv[u][v] = true;
        vvv[v][u] = true;
    }
    let mut rs = 0usize;
    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                if vvv[i][j] && vvv[j][k] && vvv[k][i] {
                    rs += 1;
                }
            }
        }
    }
    println!("{rs}");
}
