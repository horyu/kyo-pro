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
        mut xxyy: [(f64, f64); n],
    };
    xxyy.push((0.0, 0.0));
    let mut rs = 0.0;
    for ((px, py), (qx, qy)) in xxyy.into_iter().circular_tuple_windows() {
        // rs += ((px - qx).powi(2) + (py - qy).powi(2)).sqrt();
        rs += (px - qx).hypot(py - qy);
    }
    println!("{rs}");
}
