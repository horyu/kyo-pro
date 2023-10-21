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
        ss: [isize; n - 1],
        xx: [isize; m],
    };
    // https://atcoder.jp/contests/abc255/editorial/4098
    let mut bb = vec![0; n];
    for (i, s) in ss.iter().copied().enumerate() {
        bb[i + 1] = s - bb[i];
    }
    let mut counter = counter::Counter::<isize>::new();
    for (i, b) in bb.iter().copied().enumerate() {
        for (j, x) in xx.iter().copied().enumerate() {
            let k = (x - b) * (-1isize).pow(i.is_odd() as u32);
            counter[&k] += 1;
        }
    }

    let rs = counter.values().max().copied().unwrap_or_default();
    println!("{rs}");
}
