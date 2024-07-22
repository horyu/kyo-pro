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
        t: Chars,
        n: usize,
    };
    let mut sss = vec![];
    for _ in 0..n {
        input! {
            a: usize,
            ss: [Chars; a],
        };
        sss.push(ss);
    }
    let tlen = t.len();
    let mut btm = BTreeMap::new();
    btm.insert(0usize, 0usize);
    for ss in sss {
        let mut new_btm = btm.clone();
        for (k, v) in btm {
            if tlen <= k {
                continue;
            }
            for s in &ss {
                if izip!(&t[k..], s).all(|(&x, &y)| x == y) {
                    let e = new_btm.entry(k + s.len()).or_insert(!0);
                    *e = (*e).min(v + 1);
                }
            }
        }
        btm = new_btm;
    }
    if let Some(rs) = btm.get(&tlen) {
        println!("{rs}");
    } else {
        println!("-1");
    }
}
