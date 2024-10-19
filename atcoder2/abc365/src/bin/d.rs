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
        s: Chars,
    };
    let mut hm = HashMap::new();
    hm.insert('@', 0);
    for c in s {
        let mut new_hm = HashMap::new();
        for (k, v) in hm {
            // あいこ
            if k != c {
                let e = new_hm.entry(c).or_default();
                *e = v.max(*e);
            }
            // 勝ち
            let nk = match c {
                'R' => 'P',
                'S' => 'R',
                'P' => 'S',
                _ => unreachable!(),
            };
            if nk != k {
                let e = new_hm.entry(nk).or_default();
                *e = (v + 1).max(*e);
            }
        }
        hm = new_hm;
    }
    let rs = hm.values().max().unwrap();
    println!("{rs}");
}
