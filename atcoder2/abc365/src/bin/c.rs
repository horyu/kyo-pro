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
        m: u128,
        aa: [u128; n],
    };
    const MAX: u128 = 1e15 as u128;
    let mut ok = 0;
    let mut ng = MAX;
    while 1 < ng - ok {
        let mid = (ok + ng) / 2;
        let sum = aa.iter().copied().map(|a| a.min(mid)).sum::<u128>();
        if sum <= m {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    if ng == MAX {
        println!("infinite");
        return;
    }
    let rs = ok;
    println!("{rs}");
}
