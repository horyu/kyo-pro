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
        xxyyzz: [[isize; 3]; n],
    };
    let mut rs = 0;
    for ss in (0..3).map(|_| [1, -1]).multi_cartesian_product() {
        let sum = xxyyzz
            .iter()
            .map(|xyz| izip!(xyz, &ss).fold(0, |acc, (v, s)| acc + v * s))
            .sorted_unstable()
            .rev()
            .take(m)
            .sum::<isize>();
        rs = rs.max(sum);
    }
    println!("{rs}");
}
