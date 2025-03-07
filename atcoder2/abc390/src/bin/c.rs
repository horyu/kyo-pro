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
        ss: [Chars; h],
    };
    let u = (0..h).find(|&i| (0..w).any(|j| ss[i][j] == '#')).unwrap();
    let d = (0..h)
        .rev()
        .find(|&i| (0..w).any(|j| ss[i][j] == '#'))
        .unwrap();
    let l = (0..w).find(|&j| (0..h).any(|i| ss[i][j] == '#')).unwrap();
    let r = (0..w)
        .rev()
        .find(|&j| (0..h).any(|i| ss[i][j] == '#'))
        .unwrap();
    let tf = ss[u..=d]
        .iter()
        .all(|s| s[l..=r].iter().all(|&c| matches!(c, '#' | '?')));
    let rs = ["No", "Yes"][tf as usize];
    println!("{rs}");
}
