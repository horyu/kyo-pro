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
        uuvv: [(Usize1, Usize1); m],
    };
    let mut g = vec![vec![]; n];
    for (u, v) in uuvv {
        g[u].push(v);
    }
    let mut rs = 0usize;
    for i in 0..n {
        let mut qq = VecDeque::new();
        let mut pushed = vec![false; n];
        qq.push_back(i);
        pushed[i] = true;
        while let Some(q) = qq.pop_front() {
            for &j in &g[q] {
                if pushed[j] {
                    continue;
                }
                pushed[j] = true;
                qq.push_back(j);
            }
        }
        let size = pushed.into_iter().filter(|&tf| tf).count();
        // dbg!(size);
        rs += size - 1;
    }
    rs -= m;
    println!("{rs}");
}
