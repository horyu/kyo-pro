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
        xx: Bytes,
        m: usize,
    };
    let xx = xx.into_iter().map(|b| (b - b'0') as usize).collect_vec();
    if xx.len() == 1 {
        let rs = usize::from(xx[0] <= m);
        println!("{rs}");
        return;
    }
    let check = |k: usize| -> bool {
        let mut v = 0usize;
        for x in xx.iter().copied() {
            v = v.saturating_mul(k).saturating_add(x);
        }
        v <= m
    };
    let d = xx.iter().copied().max().unwrap();
    let mut ok = d + 1;
    if !check(ok) {
        println!("0");
        return;
    }
    let mut ng = 1e18 as usize + 2;
    while 1 < ng - ok {
        let mid = (ok + ng) / 2;
        if check(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    let rs = ok - d;
    println!("{rs}");
}
