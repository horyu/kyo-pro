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
        pp: [usize; n],
        qq: [usize; n],
    };
    // https://atcoder.jp/contests/arc133/editorial/3283
    let mut q2j = vec![0; n + 1];
    for (j, q) in qq.iter().copied().enumerate() {
        q2j[q] = j;
    }
    let mut ww = vec![1usize << 40; n];
    for p in pp.iter().copied() {
        let mut vv = vec![];
        for j in (p..=n).step_by(p) {
            vv.push(q2j[j]);
        }
        vv.sort_unstable();
        vv.reverse();
        for v in vv {
            let index = ww.partition_point(|&w| w < v);
            // eprintln!("{p}: {} {} {}", v, index, ww[index]);
            ww[index] = v;
        }
    }
    let rs = ww.partition_point(|&w| w < 1usize << 40);
    println!("{rs}");
}
