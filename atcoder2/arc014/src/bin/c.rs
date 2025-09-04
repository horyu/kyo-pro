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
        s: Chars
    };
    let mut hs = HashSet::new();
    hs.insert(VecDeque::default());
    for c in s.iter().copied() {
        let mut new_hs = HashSet::new();
        for mut qf in hs {
            let mut qb = qf.clone();
            if qf.front() == Some(&c) {
                qf.pop_front();
            } else {
                qf.push_front(c);
            }
            if qf.len() < 4 {
                let mut tmp = qf.clone();
                tmp.make_contiguous().reverse();
                new_hs.insert(qf.min(tmp));
            }

            if qb.back() == Some(&c) {
                qb.pop_back();
            } else {
                qb.push_back(c);
            }
            if qb.len() < 4 {
                let mut tmp = qb.clone();
                tmp.make_contiguous().reverse();
                new_hs.insert(qb.min(tmp));
            }
        }
        hs = new_hs;
    }
    let rs = hs.into_iter().map(|qq| qq.len()).min().unwrap();
    println!("{rs}");
}
