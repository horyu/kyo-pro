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
    for (u, v) in uuvv.iter().copied() {
        g[u].push(v);
        g[v].push(u);
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
    // Dangerousな頂点に対して、距離が一番近いSafeな頂点と二番目に近いSafeな頂点
    let mut d2ss = vec![[None::<usize>; 2]; n];
    // 頂点間の距離
    let mut dist = vec![vec![!0usize; n]; n];
    let mut checked_cnt = 0;
    let mut iibb = ss.iter().copied().map(|i| (i, i)).collect_vec();
    for d in 0.. {
        let mut new_iibb = vec![];
        for (i, base) in iibb {
            if dist[base][i] != !0 {
                continue;
            }
            dist[base][i] = d;
            dist[i][base] = d;
            if !ttff[i] {
                let d2s = &mut d2ss[i];
                if d2s[0].is_none() {
                    d2s[0] = Some(base);
                } else if d2s[0] != Some(base) && d2s[1].is_none() {
                    d2s[1] = Some(base);
                    checked_cnt += 1;
                }
            }
            for j in g[i].iter().copied() {
                if dist[base][j] == !0 {
                    // 多分ここで特定の頂点に対してあるsafeな頂点の個数を3個以上考える必要がない
                    new_iibb.push((j, base));
                }
            }
        }
        if checked_cnt == dd.len() {
            break;
        }
        iibb = new_iibb;
    }
    let rs = dd
        .into_iter()
        .map(|d| {
            d2ss[d]
                .iter()
                .copied()
                .flatten()
                .map(|i| dist[d][i])
                .sum::<usize>()
        })
        .join("\n");
    println!("{rs}");
}
