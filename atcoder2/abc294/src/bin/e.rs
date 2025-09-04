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
        l: Usize1,
        n1: usize,
        n2: usize,
        mut aabb: [(usize, usize); n1],
        mut ccdd: [(usize, usize); n2],
    };
    let mut rs = 0usize;
    let mut i = 0;
    let mut j = 0;
    while i < n1 && j < n2 {
        let (a, b) = aabb[i];
        let (c, d) = ccdd[j];
        if a == c {
            rs += b.min(d);
        }
        match b.cmp(&d) {
            Ordering::Less => {
                i += 1;
                ccdd[j].1 -= b;
            }
            Ordering::Equal => {
                i += 1;
                j += 1;
            }
            Ordering::Greater => {
                aabb[i].1 -= d;
                j += 1;
            }
        }
    }
    println!("{rs}");
}
