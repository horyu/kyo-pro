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
        k: usize,
        aa: [usize; n],
        bb: [usize; n],
    };
    let cnt = izip!(aa, bb).fold(0, |acc, (a, b)| acc + a.abs_diff(b));
    let rs = ["No", "Yes"][((cnt <= k) && cnt.abs_diff(k).is_even()) as usize];
    println!("{rs}");
}
