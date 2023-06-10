#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        p: char,
        q: char,
    };
    let arr = [0usize, 3, 4, 8, 9, 14, 23];
    let rs = arr[(p as u8 - b'A') as usize].abs_diff(arr[(q as u8 - b'A') as usize]);
    println!("{rs}");
}
