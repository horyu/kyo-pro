#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n],
    };
    aa.sort_unstable();
    let mut bb = vec![0; n];
    for (i, a) in aa.into_iter().enumerate() {
        if i <= n / 2 {
            bb[i * 2] = a;
        } else {
            bb[(i - n / 2) * 2 - 1] = a;
        }
    }
    let tf = bb
        .into_iter()
        .tuple_windows()
        .step_by(2)
        .all(|(a, b, c)| a < b && b > c);
    let rs = ["No", "Yes"][tf as usize];
    println!("{rs}");
}
