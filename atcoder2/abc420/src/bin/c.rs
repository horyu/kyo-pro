#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[cfg(not(debug_assertions))]
macro_rules! eprintln {
    ($($tt:tt)*) => {};
}

fn main() {
    input! {
        n: usize,
        q: usize,
        mut aa: [usize; n],
        mut bb: [usize; n],
        ccxxvv: [(char, Usize1, usize); q],
    };
    let mut rs = izip!(&aa, &bb).map(|(&a, &b)| a.min(b)).sum::<usize>();
    for (c, x, v) in ccxxvv {
        if c == 'A' {
            let old = aa[x].min(bb[x]);
            let new = v.min(bb[x]);
            rs += new;
            rs -= old;
            aa[x] = v;
        } else {
            let old = aa[x].min(bb[x]);
            let new = aa[x].min(v);
            rs += new;
            rs -= old;
            bb[x] = v;
        }
        println!("{rs}");
    }
}
