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
    let mut edge_used = vec![HashSet::new(); m];
    let mut ok_dd = HashSet::new();
    let mut iibb = ss.iter().copied().map(|s| (s, s)).collect_vec();
    for depth in 1.. {
        let mut new_iibb = vec![];
        for (i, base) in iibb {
            for (j, e_idx) in g[i].iter().copied() {
                if !edge_used[e_idx].insert(base) {
                    continue;
                }
                if !ttff[j] && !ok_dd.contains(&j) {
                    let dss = &mut d2ss[j];
                    if dss[0].0 == !0 {
                        dss[0].0 = depth;
                        dss[0].1.insert(base);
                    } else if dss[0].0 == depth {
                        dss[0].1.insert(base);
                        ok_dd.insert(j);
                    } else if dss[1].0 == !0 {
                        dss[1].0 = depth;
                        dss[1].1.insert(base);
                        ok_dd.insert(j);
                    }
                }
                new_iibb.push((j, base));
            }
        }
        if ok_dd.len() == dd.len() {
            break;
        }
        iibb = new_iibb;
    }
    let rs = dd
        .into_iter()
        .map(|d| {
            let dss = &d2ss[d];
            eprintln!("d={} dss={:?}", d, dss);
            if 2 <= dss[0].1.len() {
                dss[0].0 * 2
            } else {
                dss[0].0 + dss[1].0
            }
        })
        .join("\n");
    println!("{rs}");
}
