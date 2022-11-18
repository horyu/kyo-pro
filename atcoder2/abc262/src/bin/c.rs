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
        aa: [Usize1; n],
    };
    let mut xx = vec![];
    for (i, &a) in aa.iter().enumerate() {
        xx.push(xx.last().unwrap_or(&0) + (i == a) as usize);
    }
    let mut rs = 0usize;
    for (i, &a) in aa.iter().enumerate() {
        if i == a {
            rs += xx.last().unwrap() - xx[i];
        } else {
            if aa[a] == i && a < aa[a] {
                // eprintln!("{i} {a} {}", aa[a]);
                rs += 1;
            }
        }
    }
    // eprintln!("{}", xx.iter().join(","));
    println!("{rs}");
}
