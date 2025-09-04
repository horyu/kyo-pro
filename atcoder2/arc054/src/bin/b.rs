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
        p: f64,
    };
    // t(x) = x + p / 2^(x/1.5)
    // t'(x) = 1 + p * ln(2) / 2^(x/1.5) * ln(2) / 1.5 * 2^(x/1.5)
    // t'(x) = 0 => x = 1.5 * log2(p * ln2 / 1.5)
    let x = 0f64.max(1.5 * (p * std::f64::consts::LN_2 / 1.5).log2());
    let rs = x + p / 2f64.powf(x / 1.5);
    println!("{rs}");
}
