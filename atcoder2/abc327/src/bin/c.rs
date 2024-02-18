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
        aaa: [[usize; 9]; 9],
    };
    let tf = (0..9).all(|k| {
        aaa[k].iter().copied().all_unique() && aaa.iter().map(|a| a[k]).all_unique() && {
            let (i, j) = (k / 3 * 3, k % 3 * 3);
            (0..9).map(|kk| aaa[i + kk / 3][j + kk % 3]).all_unique()
        }
    });
    let rs = ["No", "Yes"][tf as usize];
    println!("{rs}");
}
