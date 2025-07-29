#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
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
        aabb: [(Usize1, Usize1); m],
    };
    // https://atcoder.jp/contests/abc412/editorial/13385
    let mut g = vec![vec![0; n]; n];
    for (a, b) in aabb.iter().copied() {
        g[a][b] = 1;
        g[b][a] = 1;
    }
    let mut rs = !0;
    for ee in (0..(n - 1)).flat_map(|i| ((i + 1)..n).map(move |j| (i, j))).combinations(n) {
        let mut deg = vec![0; n];
        let mut sum = 0;
        for (a, b) in ee {
            deg[a] += 1;
            deg[b] += 1;
            sum += g[a][b];
        }
        if deg.into_iter().all(|d| d == 2) {
            rs = rs.min(n + m - 2 * sum);
        }
    }
    println!("{rs}");
}

#[allow(dead_code)]
fn main2() {
    input! {
        n: usize,
        m: usize,
        aabb: [(Usize1, Usize1); m],
    };
    let mut g = vec![0; n];
    for (a, b) in aabb {
        g[a] |= 1 << b;
        g[b] |= 1 << a;
    }
    // ii の順でサイクルを作る際に元からある辺の数
    let f = |ii: &[usize]| -> i32 {
        debug_assert!(ii.len() >= 3);
        let ii = ii.iter().copied().collect_vec();
        let mut checked = 0;
        let mut tmp = 0;
        for (i, j) in ii.iter().copied().cycle().tuple_windows().take(ii.len()) {
            tmp += (g[i] & (1 << j) != 0) as i32;
            checked |= 1 << i;
        }
        tmp
    };
    let nn = n as i32;
    let mm = m as i32;
    let mut rs = i32::MAX;
    for ii in (0..n).permutations(n) {
        let v = f(&ii);
        rs = rs.min(mm - v + (nn - v));
        if 6 <= n {
            for mid in 3..=(n - 3) {
                let x = f(&ii[..mid]);
                let y = f(&ii[mid..]);
                rs = rs.min(mm - x - y + (nn - x - y));
            }
        }
    }
    println!("{rs}");
}
