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
        mg: usize,
        uuvv: [(Usize1, Usize1); mg],
        mh: usize,
        aabb: [(Usize1, Usize1); mh],
    };
    let mut ddd = vec![vec![0; n]; n];
    for i in 0..(n - 1) {
        input! { aa: [usize; n - 1 - i] };
        for (j, a) in aa.into_iter().enumerate() {
            ddd[i][i + 1 + j] = a;
            ddd[i + 1 + j][i] = a;
        }
    }
    // for dd in &ddd {
    //     eprintln!("{dd:?}");
    // }

    let mut gg = vec![HashSet::new(); n];
    for (u, v) in uuvv {
        gg[u].insert(v);
        gg[v].insert(u);
    }
    let mut gh = vec![HashSet::new(); n];
    for (a, b) in aabb {
        gh[a].insert(b);
        gh[b].insert(a);
    }

    let mut rs = !0;
    for pp in (0..n).permutations(n) {
        // グラフHの頂点iをグラフGの頂点pp[i]に対応させる
        let mut cost = 0;
        let mut added = vec![vec![false; n]; n];
        for (i, hsh) in gh.iter().enumerate() {
            let p = pp[i];
            // HにあってGにない辺のコストを加算
            for &j in hsh {
                let q = pp[j];
                if !gg[p].contains(&q) {
                    if added[p][q] || added[q][p] {
                        continue;
                    }
                    added[p][q] = true;
                    added[q][p] = true;
                    cost += ddd[i][j];
                }
            }
            // HになくGにある辺のコストを加算
            for j in 0..n {
                if hsh.contains(&j) {
                    continue;
                }
                let q = pp[j];
                if gg[p].contains(&q) {
                    if added[p][q] || added[q][p] {
                        continue;
                    }
                    added[p][q] = true;
                    added[q][p] = true;
                    cost += ddd[i][j];
                }
            }
        }
        rs = rs.min(cost);
    }
    println!("{rs}");
}
