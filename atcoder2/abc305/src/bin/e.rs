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
    for (h, gg) in pphh.into_iter().group_by(|&ph| ph.1).into_iter() {
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
        for (p, _) in gg {
            if nokori.remove(&p) {
                // eprintln!("* {pre_h}->{h}: {p}");
                checking.insert(p);
            }
        }
        pre_h = h;
    }
    while 0 < pre_h {
        pre_h -= 1;
        let mut new_checking = HashSet::new();
        for c in checking {
            for &j in &g[c] {
                if nokori.remove(&j) {
                    // eprintln!("_ {pre_h}: {c} {j}");
                    new_checking.insert(j);
                }
            }
        }
        checking = new_checking;
    }
    let rs = (0..n)
        .filter_map(|i| (!nokori.contains(&i)).then_some(i + 1))
        .collect_vec();
    println!("{}", rs.len());
    println!("{}", rs.into_iter().join(" "));
}
