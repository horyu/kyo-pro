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
        xxyy: [(isize, isize); n],
    };
    if n == 1 {
        println!("1");
        return;
    }
    let mut hm = HashMap::new();
    for (a, b) in xxyy.into_iter().sorted_unstable().tuple_combinations() {
        *hm.entry((a.0 - b.0, a.1 - b.1)).or_insert(0) += 1;
    }
    let rs = n - hm.into_values().max().unwrap();
    println!("{rs}");
}
