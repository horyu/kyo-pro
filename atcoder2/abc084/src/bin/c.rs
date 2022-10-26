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
        ccssff: [(usize, usize, usize); n - 1],
    };
    for i in 0..n {
        let mut t = 0usize;
        for &(c, s, f) in &ccssff[i..] {
            if t <= s {
                t = s + c;
            } else {
                t = t.div_ceil(f) * f + c;
            }
        }
        println!("{t}");
    }
}
