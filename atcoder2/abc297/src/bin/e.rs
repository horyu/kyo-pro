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
    // 優先度尽きキューを使ってk番目の値を取り出す
    let mut bh = BinaryHeap::new();
    let mut hs = HashSet::new();
    bh.push(R(0));
    hs.insert(0);
    let mut cnt = 0;
    while let Some(R(v)) = bh.pop() {
        if cnt == k {
            println!("{v}");
            return;
        }
        cnt += 1;
        for i in 0..n {
            let new_v = v + aa[i];
            if hs.insert(new_v) {
                bh.push(R(new_v));
            }
        }
    }
}
