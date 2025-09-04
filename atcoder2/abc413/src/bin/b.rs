#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        ss: [String; n],
    };
    let mut hs = HashSet::new();
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let s = format!("{}{}", ss[i], ss[j]);
            hs.insert(s);
        }
    }
    let rs = hs.len();
    println!("{rs}");
}
