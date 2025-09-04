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
        q: usize,
        mut aa: [usize; n],
        iikk: [(Usize1, usize); q],
    };
    let mut counter = counter::Counter::<_>::new();
    for a in aa.iter().copied() {
        counter[&a] += 1;
    }
    let mut bts = BTreeSet::from_iter(0..(2e5 as usize + 10));
    for k in counter.keys() {
        bts.remove(k);
    }
    for (i, k) in iikk.iter().copied() {
        let pre_a = aa[i];
        counter[&pre_a] -= 1;
        if counter[&pre_a] == 0 {
            bts.insert(pre_a);
        }
        aa[i] = k;
        counter[&k] += 1;
        if counter[&k] == 1 {
            bts.remove(&k);
        }
        let rs = bts.first().unwrap();
        println!("{rs}");
    }
}
