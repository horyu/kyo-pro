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
        t: usize,
        llrr: [(usize, usize); t],
    };
    for (l, r) in llrr {
        if l.ilog10() == r.ilog10() {
            println!("{}", r - l + 1);
            continue;
        }
        let base = 10usize.pow(r.ilog10());
        let mut rs = r + 1 - base;
        if r / base == 1 {
            rs += base - l.max(r % base + 1).max(r / 10 + 1);
        }
        println!("{rs}");
    }
}
