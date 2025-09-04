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
        aa: [usize; n],
    };
    let mut bts: BTreeSet<(usize, usize)> =
        aa.into_iter().enumerate().map(|(i, a)| (a, i)).collect();
    let mut rs = 0;
    while 1 < bts.len() {
        if let (Some(&(ai, i)), Some(&(aj, j))) = (bts.last(), bts.first()) {
            rs += 1;
            if i == j {
                bts.pop_first();
            } else {
                bts.pop_last();
                if !ai.divides(&aj) {
                    bts.insert((ai % aj, i));
                }
            }
        }
    }
    println!("{rs}");
}
