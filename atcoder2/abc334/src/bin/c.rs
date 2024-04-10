#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
#![feature(iter_repeat_n)]
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
        aa: [Usize1; k],
    };
    let mut cc = vec![2; n];
    for a in aa {
        cc[a] -= 1;
    }
    let dd = cc
        .iter()
        .copied()
        .enumerate()
        .flat_map(|(i, c)| std::iter::repeat_n(i, c))
        .collect_vec();
    let ll = dd
        .iter()
        .copied()
        .tuples()
        .map(|(a, b)| usize::from(a != b))
        .cumsum::<usize>()
        .collect_vec();
    let rr = dd
        .iter()
        .copied()
        .rev()
        .tuples()
        .map(|(a, b)| usize::from(a != b))
        .cumsum::<usize>()
        .collect_vec();
    // eprintln!("{ll:?}");
    // eprintln!("{rr:?}");
    let mut rs = ll.last().copied().unwrap_or(!0);
    if k.is_odd() {
        for lc in (0..=(2 * n - k)).step_by(2) {
            let rc = 2 * n - k - 1 - lc;
            // eprintln!("{lc} {rc}");
            let mut tmp = 0;
            if 0 < lc {
                tmp += ll[lc / 2 - 1];
            }
            if 0 < rc {
                tmp += rr[rc / 2 - 1];
            }
            rs = rs.min(tmp);
        }
    }

    println!("{rs}");
}
