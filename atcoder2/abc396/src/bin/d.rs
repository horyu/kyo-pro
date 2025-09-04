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
        uuvvww: [(Usize1, Usize1, usize); m],
    };
    let mut g = vec![vec![]; n];
    for (u, v, w) in uuvvww.iter().copied() {
        g[u].push((v, w));
        g[v].push((u, w));
    }
    fn dfs(
        g: &Vec<Vec<(usize, usize)>>,
        i: usize,
        v: usize,
        visited: &mut Vec<bool>,
        rs: &mut usize,
    ) {
        if i == g.len() - 1 {
            *rs = (*rs).min(v);
            return;
        }
        visited[i] = true;
        for (j, w) in g[i].iter().copied() {
            if !visited[j] {
                dfs(g, j, v ^ w, visited, rs);
            }
        }
        visited[i] = false;
    }
    let mut rs = !0;
    dfs(&g, 0, 0, &mut vec![false; n], &mut rs);
    println!("{rs}");
}

#[allow(unused)]
fn main2() {
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
