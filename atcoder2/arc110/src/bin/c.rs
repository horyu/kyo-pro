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
        mut pp: [Usize1; n],
    };
    let mut bts = BTreeSet::from_iter(0..(n - 1));
    let mut vv = vec![];
    for i in 0..(n - 1) {
        let pos = i + pp[i..].iter().position(|&p| p == i).unwrap_or_default();
        for idx in (i..pos).rev() {
            if bts.remove(&idx) {
                pp.swap(idx, idx + 1);
                vv.push(idx + 1);
            } else {
                println!("-1");
                return;
            }
        }
    }
    if vv.len() == n - 1 && pp.into_iter().enumerate().all(|(i, p)| i == p) {
        let rs = vv.iter().join("\n");
        println!("{rs}");
    } else {
        println!("-1");
    }
}
