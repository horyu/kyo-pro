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
        xxyy: [(usize, usize); n],
    };
    let x_sum = xxyy.iter().map(|(x, _)| x).sum::<usize>();
    let y_sum = xxyy.iter().map(|(_, y)| y).sum::<usize>();
    let rs = match x_sum.cmp(&y_sum) {
        Ordering::Less => "Aoki",
        Ordering::Equal => "Draw",
        Ordering::Greater => "Takahashi",
    };
    println!("{rs}");
}
