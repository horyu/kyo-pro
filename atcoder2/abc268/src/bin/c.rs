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
        n: usize,
        pp: [usize; n],
    };
    let mut vv = vec![0; n];
    for (i, p) in pp.into_iter().enumerate() {
        vv[(p + n - i - 1) % n] += 1;
        vv[(p + n - i) % n] += 1;
        vv[(p + n - i + 1) % n] += 1;
    }
    let rs = vv.into_iter().max().unwrap();
    println!("{rs}");
}
