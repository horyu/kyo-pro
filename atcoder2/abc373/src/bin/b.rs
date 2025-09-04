#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
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
        s: Bytes,
    };
    let mut c2i = [0; 26];
    for (i, b) in s.into_iter().enumerate() {
        c2i[(b - b'A') as usize] = i;
    }
    let rs = c2i
        .into_iter()
        .tuple_windows()
        .map(|(i, j)| i.abs_diff(j))
        .sum::<usize>();
    println!("{rs}");
}
