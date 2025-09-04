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
        uuvv: [(Usize1, Usize1); n-1],
        q: usize,
    };
    let mut g = vec![vec![]; n];
    for (u, v) in uuvv.iter().copied() {
        g[u].push(v);
        g[v].push(u);
    }
    // x2i[x] = 頂点xのID
    let mut x2i = vec![0; n];
    // dd[x] = 頂点xと頂点0の距離
    let mut dd = vec![0; n];
    // ll[x] = 頂点xを根とする部分木の右端のID
    let mut ll = vec![0; n];
    let mut id = 0;
    fn dfs(
        g: &[Vec<usize>],
        x2i: &mut [usize],
        dd: &mut [usize],
        ll: &mut [usize],
        id: &mut usize,
        v: usize,
        p: usize,
    ) {
        x2i[v] = *id;
        ll[v] = *id;
        *id += 1;
        if v != 0 {
            dd[v] = dd[p] + 1;
        }
        for &u in g[v].iter().filter(|&&u| u != p) {
            dfs(g, x2i, dd, ll, id, u, v);
            ll[v] = ll[u];
        }
    }
    dfs(&g, &mut x2i, &mut dd, &mut ll, &mut id, 0, !0);
    // eprintln!("x2i:{x2i:?}");
    // eprintln!(" ll:{ll:?}");
    // eprintln!(" dd:{dd:?}");
    let mut ft = ac_library::FenwickTree::new(n, 0usize);
    for i in 0..n {
        ft.add(i, 1);
    }
    // dbg!(ft.sum(..));
    // dbg!(ft.sum(0..n));
    for _ in 0..q {
        input! {t: usize};
        if t == 1 {
            input! {x: Usize1, w: usize};
            ft.add(x2i[x], w);
            continue;
        }
        input! {y: Usize1};
        let (u, v) = uuvv[y];
        let (u, v) = if dd[u] < dd[v] { (u, v) } else { (v, u) };
        let rs = ft.sum(..).abs_diff(2 * ft.sum(x2i[v]..=ll[v]));
        println!("{rs}");
    }
}
