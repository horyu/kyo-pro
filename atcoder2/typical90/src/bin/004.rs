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
        h: usize,
        w: usize,
        aaa: [[usize; w]; h],
    };
    let yoko_sum = (0..h)
        .map(|y| (0..w).fold(0, |acc, x| acc + aaa[y][x]))
        .collect_vec();
    let tate_sum = (0..w)
        .map(|x| (0..h).fold(0, |acc, y| acc + aaa[y][x]))
        .collect_vec();
    for (i, aa) in aaa.into_iter().enumerate() {
        let mut vv = vec![];
        for (j, a) in aa.into_iter().enumerate() {
            vv.push(yoko_sum[i] + tate_sum[j] - a);
        }
        println!("{}", vv.iter().join(" "));
    }
}
