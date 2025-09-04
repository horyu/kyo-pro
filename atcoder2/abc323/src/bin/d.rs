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
        n: u128,
        sscc: [(u128, u128); n],
    };
    let mut btm = BTreeMap::from_iter(sscc);
    let mut rs = 0;
    while let Some((s, c)) = btm.pop_first() {
        let c_half = c / 2;
        if 0 < c_half {
            *btm.entry(2 * s).or_default() += c_half;
        }
        rs += c % 2;
    }
    println!("{rs}");
}
