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
        m: usize,
        bb: [usize; m],
        l: usize,
        cc: [usize; l],
        q: usize,
        xx: [usize; q],
    };
    let hs: HashSet<_> = aa
        .iter()
        .copied()
        .cartesian_product(bb.iter().copied())
        .cartesian_product(cc.iter().copied())
        .map(|((a, b), c)| a + b + c)
        .collect();
    for x in xx {
        let tf = hs.contains(&x);
        let rs = ["No", "Yes"][tf as usize];
        println!("{rs}");
    }
}
