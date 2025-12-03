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
        t: usize,
        ccdd: [(usize, usize); t],
    };
    for (c, d) in ccdd {
        let mut xmin = 1;
        let mut xmax = 9;
        let mut mul = 10;
        let mut rs = 0;
        while xmin <= c + d {
            let l = xmin.max(c + 1);
            let r = xmax.min(c + d);
            if l <= r {
                let vl = c * mul + l;
                let vr = c * mul + r;
                rs += vr.sqrt() - (vl - 1).sqrt();
            }
            xmin *= 10;
            xmax = xmax * 10 + 9;
            mul *= 10;
        }
        println!("{rs}");
    }
}
