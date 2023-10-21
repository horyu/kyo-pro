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
        mut xxyyzz: [[isize; 3]; n],
    };
    let mut rs = 0isize;
    for vv in (0..3).map(|i| [1, -1]).multi_cartesian_product() {
        xxyyzz.sort_unstable_by_key(|xyz| izip!(&vv, xyz).fold(0, |acc, (v, xyz)| acc - v * xyz));
        let mut ww = [0; 3];
        for xyz in &xxyyzz[..m] {
            for (i, xyz) in xyz.iter().enumerate() {
                ww[i] += xyz;
            }
        }
        rs = rs.max(ww.into_iter().map(|w| w.abs()).sum::<isize>());
    }
    println!("{rs}");
}
