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
        i2j: [Usize1; n],
        i2q: [Usize1; n],
    };
    let mut q2i = vec![0; n];
    for (i, q) in i2q.iter().copied().enumerate() {
        q2i[q] = i;
    }
    let rs = q2i.iter().copied().map(|i| i2q[i2j[i]] + 1).join(" ");
    println!("{rs}");
}
