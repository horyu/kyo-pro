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
        t: usize,
        aa: [usize; n],
    };
    if n == 1 {
        let rs = if aa[0] <= t { aa[0] } else { 0 };
        println!("{rs}");
        return;
    }
    // https://algo-logic.info/split-and-list/
    let half = n / 2;
    let mut bb = vec![];
    for bit in 0..(1 << half) {
        let mut sum = 0;
        for i in 0..half {
            let mask = 1 << i;
            if 0 < bit & mask {
                sum += aa[i];
            }
        }
        bb.push(sum);
    }
    let mut cc = vec![];
    for bit in 0..(1 << (n - half)) {
        let mut sum = 0;
        for i in 0..(n - half) {
            let mask = 1 << i;
            if 0 < bit & mask {
                sum += aa[half + i];
            }
        }
        cc.push(sum);
    }

    let dd = BTreeSet::from_iter(cc);
    let mut rs = 0;
    for b in bb {
        if b <= t {
            let d = dd
                .range(..=(t - b))
                .next_back()
                .copied()
                .unwrap_or_default();
            rs = rs.max(b + d);
        }
    }
    println!("{rs}");
}
