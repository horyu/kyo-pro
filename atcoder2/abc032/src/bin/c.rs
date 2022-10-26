#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
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
    if aa.iter().all(|&a| k < a) {
        println!("0");
        return;
    }
    let mut rs = 1usize;
    let mut crr = 1usize;
    let mut l = 0;
    for r in 0..n {
        crr *= aa[r];
        while l < r && k < crr {
            crr /= aa[l];
            l += 1;
        }
        rs = rs.max(r - l + 1);
    }
    println!("{rs}");
}
