#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        a: f64,
        b: f64,
    };
    let mut rs = a;
    let n = (a / 2.0 / b).powf(2.0 / 3.0) - 1.0;
    for diff in (-1)..=1 {
        let nn = (n + diff as f64).floor();
        if 0.0 < nn {
            let tmp = b * nn + a / (1.0 + nn).sqrt();
            rs = rs.min(tmp);
        }
    }
    println!("{rs}");
}
