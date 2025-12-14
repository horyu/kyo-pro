#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[cfg(not(debug_assertions))]
macro_rules! eprintln {
    ($($tt:tt)*) => {};
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uuvv: [(Usize1, Usize1); m],
        s: Chars,
    };
    let mut g = vec![vec![]; n];
    for (e_idx, (u, v)) in uuvv.iter().copied().enumerate() {
        g[u].push((v, e_idx));
        g[v].push((u, e_idx));
    }
    let ttff = s.iter().copied().map(|c| c == 'S').collect_vec();
    let (ss, dd) =
        ttff.iter()
            .copied()
            .enumerate()
            .fold((vec![], vec![]), |(mut ss, mut dd), (i, tf)| {
                if tf {
                    ss.push(i);
                } else {
                    dd.push(i);
                }
                (ss, dd)
            });
    // Dangerousな頂点に対して、距離 + 距離が一番近いSafeな頂点群と二番目に近いSafeな頂点群
    let mut d2ss = vec![vec![(!0, HashSet::<usize>::new()); 2]; n];
    let mut node_cnts = vec![0; n];
    let mut edge_cnts = vec![0; m];
    let mut iibb = ss.iter().copied().map(|s| (s, s)).collect_vec();
    for depth in 1.. {
        // TODO
        // 同じ辺を通る回数は2回までに制限する
    }
    // println!("{rs}");
}
