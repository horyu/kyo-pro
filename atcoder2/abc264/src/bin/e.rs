#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings, map_first_last)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        e: usize,
        uuvv: [(Usize1, Usize1); e],
        q: usize,
        xx: [Usize1; q],
    };
    let uuvv = uuvv
        .into_iter()
        .map(|(u, v)| (u.min(n), v.min(n)))
        .collect_vec();

    let mut dsu = ac_library_rs::Dsu::new(n + 1);
    let xx_hs: HashSet<usize> = xx.clone().into_iter().collect();
    for (i, &(u, v)) in uuvv.iter().enumerate() {
        if xx_hs.contains(&i) {
            continue;
        }
        dsu.merge(u, v);
    }

    let mut rs = vec![0; q];
    for qq in (0..q).rev() {
        rs[qq] = dsu.size(n) - 1;
        let (u, v) = uuvv[xx[qq]];
        dsu.merge(u, v);
    }
    let rs = rs.into_iter().join("\n");
    println!("{rs}");
}
