#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
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
        n: usize,
        dd: [usize; n - 1],
    };
    let pp = chain([0], dd).cumsum::<usize>().collect_vec();
    for (i, pi) in pp[..(n - 1)].iter().copied().enumerate() {
        let rs = pp[(i + 1)..].iter().copied().map(|pj| pj - pi).join(" ");
        println!("{rs}");
    }
}
