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
        k: usize,
        aa: [usize; n],
    };
    // https://atcoder.jp/contests/abc393/editorial/12243
    let max = aa.iter().copied().max().unwrap();
    let mut ss = vec![0; max + 1];
    let mut tt = ss.clone();
    let mut uu = ss.clone();
    for a in aa.iter().copied() {
        ss[a] += 1;
    }
    for d in 1..=max {
        for n in (d..=max).step_by(d) {
            tt[d] += ss[n];
        }
    }
    for d in 1..=max {
        if tt[d] < k {
            continue;
        }
        for n in (d..=max).step_by(d) {
            uu[n] = uu[n].max(d);
        }
    }
    for a in aa {
        let rs = uu[a];
        println!("{rs}");
    }
}
