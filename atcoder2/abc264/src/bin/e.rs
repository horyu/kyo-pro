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
        e: usize,
        uuvv: [(Usize1, Usize1); e],
        q: usize,
        xx: [Usize1; q],
    };
    // 全ての都市 n + 集約した発電所 1 で考える
    let uuvv = uuvv
        .into_iter()
        .map(|(u, v)| (u.min(n), v.min(n)))
        .collect_vec();
    let xhs = xx.iter().copied().collect::<HashSet<_>>();

    let mut dsu = ac_library::Dsu::new(n + 1);
    for (i, (u, v)) in uuvv.iter().copied().enumerate() {
        if !xhs.contains(&i) {
            dsu.merge(u, v);
        }
    }
    let mut rrss = vec![];
    for x in xx.into_iter().rev() {
        rrss.push(dsu.size(n) - 1);
        let (u, v) = uuvv[x];
        dsu.merge(u, v);
    }
    let rs = rrss.into_iter().rev().join("\n");
    println!("{rs}");
}
