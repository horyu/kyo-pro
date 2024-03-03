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
        q: usize,
        ss: Chars,
        llrr: [(usize, usize); q],
    };
    let mut cc = vec![0; n + 1];
    for (i, (cx, cy)) in ss.iter().copied().tuple_windows().enumerate() {
        cc[i + 1] = cc[i];
        if cx == cy {
            cc[i + 1] += 1;
        }
    }
    for (l, r) in llrr {
        let rs = cc[r - 1] - cc[l - 1];
        println!("{rs}");
    }
}
