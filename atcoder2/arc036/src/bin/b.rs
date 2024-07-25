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
        hh: [usize; n],
    };
    // 左から右に見ていく
    let mut ll = vec![0; n];
    for (i, (hl, hr)) in hh.iter().copied().tuple_windows().enumerate() {
        if hl < hr {
            ll[i + 1] = ll[i] + 1;
        }
    }
    // 右から左に見ていく
    let mut rr = vec![0; n];
    for (i, (hr, hl)) in hh.iter().copied().rev().tuple_windows().enumerate() {
        if hr < hl {
            rr[n - 2 - i] = rr[n - 1 - i] + 1;
        }
    }
    let rs = 1 + izip!(ll, rr).map(|(l, r)| l + r).max().unwrap();
    println!("{rs}");
}
