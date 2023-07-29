#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        mut m: usize,
        ttxx: [(usize, usize); n],
    };
    let mut tmp = [vec![], vec![], vec![]];
    for (t, x) in ttxx {
        tmp[t].push(x);
    }
    let [aa, mut bb, mut cc] = tmp;
    use std::cmp::Reverse as R;
    let mut bh = BinaryHeap::new();
    let mut sum = 0;
    for a in aa.into_iter().sorted_unstable().rev().take(m) {
        bh.push(R(a));
        sum += a;
    }

    bb.sort_unstable();
    cc.sort_unstable();
    let mut rs = sum;
    for (ci, c) in cc.into_iter().rev().enumerate() {
        for _ in 0..c {
            if let Some(b) = bb.pop() {
                bh.push(R(b));
                sum += b;
            } else {
                break;
            }
        }
        while m - ci - 1 < bh.len() {
            if let Some(R(x)) = bh.pop() {
                sum -= x;
            }
        }
        rs = rs.max(sum);
    }
    println!("{rs}");
}
