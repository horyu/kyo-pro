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
    // https://atcoder.jp/contests/abc245/editorial/3664
    let mut g = vec![vec![]; n];
    for (u, v) in uuvv.iter().copied() {
        g[u].push(v);
    }
    let vvv = pathfinding::directed::strongly_connected_components::strongly_connected_components(
        &(0..n).collect_vec(),
        |&i| g[i].clone(),
    );
    let k = vvv.len();
    let mut jj = vec![0; n];
    for (i, vv) in vvv.iter().enumerate() {
        for &v in vv {
            jj[v] = i;
        }
    }

    let mut rs = 0;
    let mut dp = vec![false; k];
    for (i, vv) in vvv.iter().enumerate() {
        if vv.len() == 1 {
            for v in g[vv[0]].iter().copied() {
                dp[i] |= dp[jj[v]];
            }
        } else {
            dp[i] = true;
        }
        if dp[i] {
            rs += vv.len();
        }
    }
    println!("{rs}");
}
