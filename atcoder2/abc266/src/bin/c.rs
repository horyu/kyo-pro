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
        xxyy: [(f64, f64); 4],
    };
    let cal = |i: usize, j: usize, k: usize| -> f64 {
        ((xxyy[i].0 - xxyy[k].0) * (xxyy[j].1 - xxyy[k].1)
            - (xxyy[j].0 - xxyy[k].0) * (xxyy[i].1 - xxyy[k].1))
            .signum()
    };
    let tf = (0..4)
        .cycle()
        .tuple_windows()
        .take(4)
        .all(|(i, j, k)| !cal(i, j, k).is_sign_negative());
    let rs = ["No", "Yes"][tf as usize];
    println!("{rs}");
}
