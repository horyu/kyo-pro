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
        q: usize,
        s: Chars,
        ttllrr: [(usize, Usize1, Usize1); q],
    };
    let ttff = s.into_iter().map(|c| c == '0').collect_vec();
    let mut bts = BTreeSet::new();
    for (i, (a, b)) in ttff.iter().copied().tuple_windows().enumerate() {
        if a == b {
            bts.insert(i);
        }
    }
    for (t, l, r) in ttllrr {
        if t == 1 {
            if 0 < l && !bts.remove(&(l - 1)) {
                bts.insert(l - 1);
            }
            if !bts.remove(&r) {
                bts.insert(r);
            }
            // eprintln!("{:?}", bts);
        } else {
            let tf = bts.range(l..r).next().is_none();
            let rs = ["No", "Yes"][tf as usize];
            println!("{rs}");
        }
    }
}
