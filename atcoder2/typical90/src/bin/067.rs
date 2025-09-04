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
        n: String,
        k: usize,
    };
    let mut rs = usize::from_str_radix(&n, 8).unwrap_or_default();
    for _ in 0..k {
        let mut vv = vec![];
        let mut tmp = rs;
        while 0 < tmp {
            vv.push(tmp % 9);
            tmp /= 9;
        }
        rs = vv
            .into_iter()
            .rev()
            .fold(0, |acc, v| acc * 8 + if v == 8 { 5 } else { v });
    }
    println!("{rs:0o}");
}
