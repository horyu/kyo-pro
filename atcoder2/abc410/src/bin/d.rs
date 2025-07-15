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
        n: usize,
        m: usize,
        aabbww: [(Usize1, Usize1, usize); m],
    };
    let mut g = vec![vec![]; n];
    for (a, b, w) in aabbww.iter().copied() {
        g[a].push((b, w));
    }
    let mut pushed = vec![vec![false; 2usize << 10]; n];
    let mut qq = VecDeque::new();
    qq.push_back((0, 0));
    pushed[0][0] = true;
    while let Some((qi, qw)) = qq.pop_front() {
        for (i, w) in g[qi].iter().copied() {
            let ww = qw ^ w;
            if !pushed[i][ww] {
                pushed[i][ww] = true;
                qq.push_back((i, ww));
            }
        }
    }
    if let Some(rs) = pushed[n - 1].iter().position(|&tf| tf) {
        println!("{rs}");
    } else {
        println!("-1");
    }
}
