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
    };
    let mut cc = vec!['-'; n];
    let h = n / 2;
    if n % 2 == 0 {
        cc[h - 1] = '=';
        cc[h] = '=';
    } else {
        cc[h] = '=';
    };
    let rs = cc.iter().join("");
    println!("{rs}");
}
