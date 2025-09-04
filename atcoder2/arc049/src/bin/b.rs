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
        xxyycc: [(f64, f64, f64); n],
    };
    let mut l = 0.0;
    let mut r = 1e9;
    for _ in 0..64 {
        let m = (l + r) / 2.0;
        let mut xd = f64::MIN;
        let mut xu = f64::MAX;
        let mut yl = f64::MIN;
        let mut yr = f64::MAX;
        for (x, y, c) in xxyycc.iter().copied() {
            xd = xd.max(x - m / c);
            xu = xu.min(x + m / c);
            yl = yl.max(y - m / c);
            yr = yr.min(y + m / c);
        }
        if xd <= xu && yl <= yr {
            r = m;
        } else {
            l = m;
        }
    }
    let rs = l;
    println!("{rs:.15}");
}
