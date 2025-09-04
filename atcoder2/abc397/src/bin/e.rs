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
        k: usize,
        uuvv: [(Usize1, Usize1); n * k - 1],
    };
    let mut g = vec![vec![]; n * k];
    for (u, v) in uuvv.iter().copied() {
        g[u].push(v);
        g[v].push(u);
    }
    // https://atcoder.jp/contests/abc397/submissions/63550824
    let mut vv = vec![(0, !0, 0)];
    // 部分木の大きさ
    let mut sizes = vec![1; n * k];
    while let Some((cur, from, t)) = vv.pop() {
        if t == 0 {
            vv.push((cur, from, 1));
            for next in g[cur].iter().copied() {
                if next != from {
                    vv.push((next, cur, 0));
                }
            }
            continue;
        }
        // 子ノードの数
        let mut cnt = 0;
        for next in g[cur].iter().copied() {
            if next != from {
                sizes[cur] += sizes[next];
                if 0 < sizes[next] {
                    cnt += 1;
                }
            }
        }
        if k < sizes[cur] || 3 <= cnt {
            println!("No");
            return;
        }
        if sizes[cur] < k && 2 <= cnt {
            println!("No");
            return;
        }
        if sizes[cur] == k {
            sizes[cur] = 0;
        }
    }
    println!("Yes");
}
