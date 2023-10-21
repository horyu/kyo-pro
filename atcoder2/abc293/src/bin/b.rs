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
        aa: [Usize1; n],
    };
    let mut ttff = vec![false; n];
    for (i, a) in aa.into_iter().enumerate() {
        if !ttff[i] {
            ttff[a] = true;
        }
    }
    let rs = (1..=n).filter(|i| !ttff[i - 1]).collect_vec();
    println!("{}\n{}", rs.len(), rs.iter().join(" "));
}
