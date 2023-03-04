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
    let (a, b) = (a.min(b), a.max(b));
    let rs = (a * (4.0 / 3.0f64).sqrt())
        .min((4.0 * b.powi(2) - 4.0 * 3.0f64.sqrt() * a * b + 4.0 * a.powi(2)).sqrt());
    println!("{rs}");
}
