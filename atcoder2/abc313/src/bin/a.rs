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
        pp: [usize; n],
    };
    let mut counter = counter::Counter::<usize>::new();
    let mut max = 0;
    for p in pp.iter().copied() {
        counter[&p] += 1;
        max = max.max(p);
    }
    let rs = if pp[0] == max && counter[&max] == 1 {
        0
    } else {
        max + 1 - pp[0]
    };
    println!("{rs}");
}
