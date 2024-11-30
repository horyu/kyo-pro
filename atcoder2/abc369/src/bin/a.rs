#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
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
        a: isize,
        b: isize,
    };
    let mut hs = HashSet::new();
    for x in -100..=200 {
        if [a, b, x]
            .iter()
            .permutations(3)
            .any(|vv| vv[0] - vv[1] == vv[1] - vv[2])
        {
            hs.insert(x);
        }
    }
    let rs = hs.len();
    println!("{rs}");
}
