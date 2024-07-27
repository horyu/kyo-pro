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
        n: usize,
        x: usize,
        y: usize,
        aa: [usize; n],
        bb: [usize; n],
    };
    let size = std::cmp::min(
        aa.into_iter()
            .sorted_unstable()
            .rev()
            .cumsum::<usize>()
            .take_while(|&s| s <= x)
            .count(),
        bb.into_iter()
            .sorted_unstable()
            .rev()
            .cumsum::<usize>()
            .take_while(|&s| s <= y)
            .count(),
    );
    let rs = size + usize::from(n != size);
    println!("{rs}");
}
