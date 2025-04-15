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
        uuvvww: [(Usize1, Usize1, usize); m],
    };
    let mut g = vec![vec![!0usize; n]; n];
    for (u, v, w) in uuvvww.iter().copied() {
        g[u][v] = w;
        g[v][u] = w;
    }
    let mut rs = !0;
    // 頂点0 -> (1~n-1)の並び替え -> 頂点n-1
    for size in 0..(n - 1) {
        for vv in (1..(n - 1)).permutations(size) {
            let mut tmp = 0;
            let mut ok = true;
            for (i, j) in chain!([0], vv, [n - 1]).tuple_windows() {
                if g[i][j] == !0 {
                    ok = false;
                    break;
                }
                tmp ^= g[i][j];
            }
            if ok {
                rs = rs.min(tmp);
            }
        }
    }
    println!("{rs}");
}
