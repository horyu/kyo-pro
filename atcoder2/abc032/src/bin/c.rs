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
        k: usize,
        aa: [usize; n],
    };
    if aa.contains(&0) {
        println!("{n}");
        return;
    }
    // 尺取法
    let mut rs = 0;
    for bb in aa.split(|&a| k < a) {
        let mut l = 0;
        let mut crr = 1;
        for r in 0..bb.len() {
            crr *= bb[r];
            while k < crr {
                crr /= bb[l];
                l += 1;
            }
            rs = rs.max(r - l + 1);
        }
    }
    println!("{rs}");
}
