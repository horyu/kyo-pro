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
        m: usize,
        aa: [usize; m],
        xxx: [[usize; m]; n],
    };
    let mut bb = vec![0; m];
    for xx in xxx {
        for (i, x) in xx.into_iter().enumerate() {
            bb[i] += x;
        }
    }
    let tf = izip!(aa, bb).all(|(a, b)| a <= b);
    let rs = ["No", "Yes"][tf as usize];
    println!("{rs}");
}
