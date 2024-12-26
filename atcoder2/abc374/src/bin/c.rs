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
        kk: [usize; n],
    };
    let mut rs = !0;
    for bits in 0..(1 << n) {
        let mut arr = [0; 2];
        for (i, k) in kk.iter().copied().enumerate() {
            arr[(bits >> i) & 1] += k;
        }
        rs = rs.min(arr.iter().copied().max().unwrap());
    }
    println!("{rs}");
}
