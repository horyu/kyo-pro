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

fn main() {
    input! {
        t: usize,
        nn: [usize; t],
    };
    let bts = BTreeSet::from_iter(
        (0..64)
            .map(|i| 1usize << i)
            .combinations(3)
            .map(|vv| vv[0] | vv[1] | vv[2]),
    );
    for n in nn {
        if let Some(rs) = bts.range(..=n).last() {
            println!("{rs}");
        } else {
            println!("-1");
        }
    }
}
