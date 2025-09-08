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
    };
    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            aa: [usize; n],
            bb: [usize; n],
        };
        let mut btm = BTreeMap::new();
        for a in aa {
            *btm.entry(a % m).or_insert(0) += 1;
        }
        let mut rs = 0usize;
        for b in bb {
            let b = b % m;
            let c = m - b;
            let k = *btm
                .range(c..)
                .next()
                .or_else(|| btm.range(..).next())
                .unwrap()
                .0;
            rs += (k + b) % m;
            let e = btm.get_mut(&k).unwrap();
            *e -= 1;
            if *e == 0 {
                btm.remove(&k);
            }
        }
        println!("{rs}");
    }
}
