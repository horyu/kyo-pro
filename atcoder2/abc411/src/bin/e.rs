#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

/*
3面ダイス3つで考える
a:1  45
b: 2 4  7
c:  3  67
P(X) = ダイスの最大値がXとなる組み合わせ
P(1) = 0; P(2) = 0; P(3) = 1;
P(4) = 1+2 = 3; P(5) = 2;
P(6) = 3*2 = 6; P(7) = 3*2+3*3 = 15;
*/

fn main() {
    input! {
        n: usize,
        aaa: [[usize; 6]; n],
    };
    let a2b = aaa
        .iter()
        .flatten()
        .copied()
        .sorted_unstable()
        .dedup()
        .enumerate()
        .map(|(a, b)| (b, a))
        .collect::<HashMap<_, _>>();
    let bbb = aaa
        .iter()
        .map(|a| a.iter().map(|&b| a2b[&b]).collect_vec())
        .collect_vec();

    // println!("{rs}");
}
