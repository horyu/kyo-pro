#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    const N: usize = 100;
    const HN: usize = N / 2;
    let mut rs = vec![vec!['.'; N]; HN];
    rs.extend(vec![vec!['#'; N]; HN]);
    for i in 1..b {
        rs[1 + (i / 50) * 2][2 * (i % 50)] = '#';
    }
    for i in 1..a {
        rs[HN + 1 + (i / 50) * 2][2 * (i % 50)] = '.';
    }
    println!("{N} {N}");
    for rs in rs {
        println!("{}", rs.iter().join(""));
    }
}
