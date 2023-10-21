#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
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
        uuvv: [(Usize1, Usize1); m],
    };
    let mut dsu = ac_library::Dsu::new(n);
    let mut hm = HashMap::new();
    for (u, v) in uuvv.iter().copied() {
        dsu.merge(u, v);
        *hm.entry(u.min(v)).or_insert(0) += 1;
    }

    let tf = dsu.groups().into_iter().all(|gg| {
        let len = gg.len();
        let mut cnt = 0usize;
        for g in gg {
            cnt += hm.get(&g).unwrap_or(&0);
        }
        len == cnt
    });

    let rs = ["No", "Yes"][tf as usize];
    println!("{rs}");
}
