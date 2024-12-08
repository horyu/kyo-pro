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
    };
    let mut aaa = vec![];
    for i in 1..=n {
        input! {aa: [Usize1; i]};
        aaa.push(aa);
    }
    let mut rs = 0;
    for i in 0..n {
        rs = aaa[rs.max(i)][rs.min(i)];
    }
    rs += 1;
    println!("{rs}");
}
