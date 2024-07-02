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
        llddkkccaabb: [(usize, usize, usize, usize, Usize1, Usize1); m],
    };
    let mut ff = vec![vec![]; n];
    for (l, d, k, c, a, b) in llddkkccaabb {
        ff[b].push((a, l, d, k, c));
    }
    let mut poped = vec![0; n];

    let mut bh = BinaryHeap::new();
    bh.push((1 << 60, n - 1));
    while let Some((qt, qi)) = bh.pop() {
        if poped[qi] != 0 {
            continue;
        }
        poped[qi] = qt;
        for (ni, l, d, k, c) in ff[qi].iter().copied() {
            if poped[ni] != 0 || qt < l + c {
                continue;
            }
            // l + k_max * d + c <= qt
            // k_max <= (qt - l - c) / d
            let kk = (k - 1).min((qt - l - c) / d);
            let nt = l + kk * d;
            // eprintln!("{start} {qi}->{ni} {qt}:{nt}");
            bh.push((nt, ni));
        }
    }
    for p in poped[..(n - 1)].iter().copied() {
        if p == 0 {
            println!("Unreachable");
            continue;
        }
        println!("{p}");
    }
}
