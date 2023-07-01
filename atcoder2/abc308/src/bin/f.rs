#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut pp: [usize; n],
        ll: [usize; m],
        dd: [usize; m],
    };
    pp.sort_unstable();
    let mut lldd = izip!(ll, dd).sorted_unstable().collect::<VecDeque<_>>();
    let mut rs = 0;
    let mut bh = BinaryHeap::new();
    for p in pp {
        while let Some((l, d)) = lldd.front().copied() {
            if l <= p {
                lldd.pop_front();
                bh.push(d);
            } else {
                break;
            }
        }
        rs += p - bh.pop().unwrap_or_default();
    }
    println!("{rs}");
}
