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
        xxyyzz: [(usize, usize, usize); n],
    };
    let sum_z = xxyyzz.iter().map(|(_, _, z)| z).sum::<usize>();
    let mut vv = vec![1usize << 60; sum_z + 1];
    vv[0] = 0;
    for (x, y, z) in xxyyzz {
        for i in (0..=(sum_z - z)).rev() {
            vv[i + z] = vv[i + z].min(vv[i] + y.saturating_sub(x).div_ceil(2));
        }
    }
    let rs = vv[sum_z.div_ceil(2)..].iter().min().unwrap();
    println!("{rs}");
}
