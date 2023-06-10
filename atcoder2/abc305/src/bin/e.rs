#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        uuvv: [(Usize1, Usize1); m],
        mut pphh: [(Usize1, usize); k],
    };
    let mut g = vec![vec![]; n];
    for (u, v) in uuvv.iter().copied() {
        g[u].push(v);
        g[v].push(u);
    }

    pphh.sort_unstable_by_key(|ph| std::cmp::Reverse(ph.1));

    let mut checking = HashSet::new();
    let mut nokori: HashSet<usize> = (0..n).collect();
    let mut pre_h = 0;
    let vv = pphh
        .into_iter()
        .group_by(|&ph| ph.1)
        .into_iter()
        .map(|(h, pphh)| (h, pphh.map(|ph| ph.0).collect_vec()))
        .collect_vec();
    for (h, pp) in vv.into_iter().chain([Default::default()]) {
        while h < pre_h {
            pre_h -= 1;
            let mut new_checking = HashSet::new();
            for c in checking {
                for &j in &g[c] {
                    if nokori.remove(&j) {
                        // eprintln!("@ {pre_h}: {c} {j}");
                        new_checking.insert(j);
                    }
                }
            }
            checking = new_checking;
        }
        for p in pp {
            if nokori.remove(&p) {
                // eprintln!("* {pre_h}->{h}: {p}");
                checking.insert(p);
            }
        }
        pre_h = h;
    }
    let rs = (0..n)
        .filter_map(|i| (!nokori.contains(&i)).then_some(i + 1))
        .collect_vec();
    println!("{}", rs.len());
    println!("{}", rs.into_iter().join(" "));
}
