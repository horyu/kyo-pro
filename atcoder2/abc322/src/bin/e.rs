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
        k: usize,
        p: usize,
        mut caa: [(usize, [usize; k]); n],
    };
    let mut rs = usize::MAX;
    let mut hm = HashMap::new();
    hm.insert(vec![0; k], 0);
    for (c, aa) in caa {
        let mut new_hm = hm.clone();
        for (mut base_aa, cost) in hm {
            for (i, a) in aa.iter().enumerate() {
                base_aa[i] = (base_aa[i] + a).min(p);
            }
            if base_aa.iter().copied().all(|a| a == p) {
                rs = rs.min(cost + c);
            }
            let e = new_hm.entry(base_aa).or_insert(!0);
            *e = (*e).min(cost + c);
        }
        hm = new_hm
    }
    if rs == usize::MAX {
        println!("-1")
    } else {
        println!("{rs}");
    }
}
