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
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            x: Usize1,
            y: Usize1,
            uuvv: [(Usize1, Usize1); m],
        };
        let mut g = vec![vec![]; n];
        for (u, v) in uuvv.iter().copied() {
            g[u].push(v);
            g[v].push(u);
        }
        // TODO: 多分xとyのパスの間にない一方通行の辺を消す
        for vv in g.iter_mut() {
            vv.sort_unstable();
        }
        let mut rrss = vec![x + 1];
        let mut visited = vec![false; n];
        visited[x] = true;
        fn dfs(
            g: &Vec<Vec<usize>>,
            visited: &mut Vec<bool>,
            rrss: &mut Vec<usize>,
            v: usize,
            y: usize,
        ) -> bool {
            if v == y {
                return true;
            }
            for w in g[v].iter().copied() {
                if visited[w] {
                    continue;
                }
                visited[w] = true;
                rrss.push(w + 1);
                if dfs(g, visited, rrss, w, y) {
                    return true;
                }
                rrss.pop();
                visited[w] = false;
            }
            false
        }
        dfs(&g, &mut visited, &mut rrss, x, y);
        let rs = rrss.iter().join(" ");
        println!("{rs}");
    }
}
