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
        q: usize,
        cc: [Usize1; n],
        qq: [(usize, Usize1, Usize1); q],
    };
    let mut uf = UnionFind::new(n);
    let mut hhmm = vec![HashMap::new(); n];
    for (i, c) in cc.iter().copied().enumerate() {
        hhmm[i].insert(c, 1usize);
    }
    for (q_type, a, b) in qq {
        if q_type == 1 {
            let ia = uf.find(a);
            let ib = uf.find(b);
            if ia == ib {
                continue;
            }
            let (sml_i, big_i) = if hhmm[ia].len() < hhmm[ib].len() {
                (ia, ib)
            } else {
                (ib, ia)
            };
            let sml_hm = std::mem::take(&mut hhmm[sml_i]);
            for (k, v) in sml_hm {
                *hhmm[big_i].entry(k).or_insert(0) += v;
            }

            uf.union(a, b);
            if sml_i == uf.find(a) {
                hhmm.swap(sml_i, big_i);
            }
        } else {
            let rs = hhmm[uf.find(a)].get(&b).copied().unwrap_or_default();
            println!("{rs}");
        }
    }
}
