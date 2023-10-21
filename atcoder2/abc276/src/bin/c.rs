#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
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
        pp: [usize; n],
    };
    // 右から左に見て初めて大きくなる所
    let l = (0..(n - 1)).rposition(|i| pp[i + 1] < pp[i]).unwrap();
    dbg!(l);
    let vv = pp[l..].to_owned();
    eprintln!("{}", vv.iter().join(" "));
    let next = *vv.iter().filter(|&&v| v < pp[l]).max().unwrap();
    dbg!(next);
    let rr = vv
        .into_iter()
        .filter(|&v| v != next)
        .sorted()
        .rev()
        .collect_vec();
    eprintln!("{}", rr.iter().join(" "));

    let mut rs = vec![];
    for i in 0..l {
        rs.push(pp[i]);
    }
    rs.push(next);
    rs.extend(&rr);
    println!("{}", rs.iter().join(" "));
}
