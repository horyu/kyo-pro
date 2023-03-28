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
        n: usize,
        ww: [String; n],
    };
    let tf = ww
        .into_iter()
        .any(|w| matches!(w.as_str(), "and" | "not" | "that" | "the" | "you"));
    let rs = ["No", "Yes"][tf as usize];
    println!("{rs}");
}