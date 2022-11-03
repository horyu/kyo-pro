#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        xxyy: [(Usize1, Usize1); m]
    };
    let mut ttff = vec![vec![false; n]; n];
    for (x, y) in xxyy {
        ttff[x][y] = true;
        ttff[y][x] = true;
    }
    for size in (2..=n).rev() {
        for ii in (0..n).combinations(size) {
            if ii.into_iter().tuple_combinations().all(|(i, j)| ttff[i][j]) {
                println!("{size}");
                return;
            }
        }
    }
    println!("1");
}
