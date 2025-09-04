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
        q: usize,
        aa: [usize; n],
        xxyy: [(Usize1, Usize1); q],
    };
    let mut ngs = vec![vec![false; n]; n];
    for (x, y) in xxyy {
        ngs[x][y] = true;
        ngs[y][x] = true;
    }
    let mut hm = HashMap::new();
    hm.insert(0, Vec::<usize>::new());
    for (i, a) in aa.iter().copied().enumerate() {
        let mut new_hm = hm.clone();
        for (k, mut jj) in hm {
            if jj.iter().any(|&j| ngs[i][j]) {
                continue;
            }
            jj.push(i);
            let new_k = k + a;
            if let Some(kk) = new_hm.remove(&new_k) {
                for xx in [kk, jj] {
                    println!("{}\n{}", xx.len(), xx.iter().map(|x| 1 + x).join(" "));
                    // eprintln!("{}", xx.iter().copied().map(|x| aa[x]).sum::<usize>());
                }
                return;
            }
            new_hm.insert(new_k, jj);
        }
        hm = new_hm;
    }
}
