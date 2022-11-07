#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        l: usize,
        r: usize,
        s: Chars
    };
    println!(
        "{}{}{}",
        &s[..(l - 1)].iter().join(""),
        &s[(l - 1)..r].iter().rev().join(""),
        &s[r..].iter().join(""),
    );
}
