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
        aa: [usize; n],
        uuvv: [(Usize1, Usize1); m],
    };
    let mut g = vec![vec![]; n];
    let mut dsu = ac_library::Dsu::new(n);
    for (u, v) in uuvv.iter().copied() {
        if aa[u] == aa[v] {
            dsu.merge(u, v);
        }
    }
    let ll = (0..n).map(|u| dsu.leader(u)).collect_vec();
    for (u, v) in uuvv.iter().copied() {
        let u = ll[u];
        let v = ll[v];
        match aa[u].cmp(&aa[v]) {
            Ordering::Less => {
                g[u].push(v);
            }
            Ordering::Greater => {
                g[v].push(u);
            }
            Ordering::Equal => {}
        }
    }

    let mut dd = vec![0; n];
    let mut bh = BinaryHeap::new();
    dd[ll[0]] = 1;
    bh.push((R(aa[ll[0]]), dd[ll[0]], ll[0]));

    while let Some((R(crr), qd, qi)) = bh.pop() {
        // eprintln!("{crr} {qd} {qi}");
        if dd[qi] < qd {
            continue;
        }
        for &i in &g[qi] {
            if dd[i] < qd + 1 {
                dd[i] = qd + 1;
                bh.push((R(aa[i]), dd[i], i));
            }
        }
    }
    let rs = dd[ll[n - 1]];
    println!("{rs}");
}
