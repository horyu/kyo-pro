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
    };
    let mut hm = HashMap::new();
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
            cc: [Usize1; b],
        };
        let d = cc.iter().fold(0u16, |acc, c| acc | (1 << c));

        let mut new_hm = hm.clone();
        for (k, v) in hm {
            let new_k = k | d;
            if new_k != d {
                let e = new_hm.entry(new_k).or_insert(!0);
                *e = (*e).min(v + a);
            }
        }

        let e = new_hm.entry(d).or_insert(!0);
        *e = a.min(*e);

        hm = new_hm;
    }
    if let Some(rs) = hm.get(&((1 << n) - 1)) {
        println!("{rs}");
    } else {
        println!("-1");
    }
}
