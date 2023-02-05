#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::{matrix_graph::Zero, unionfind::UnionFind};
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        k: usize,
        aa: [isize; n],
        q: usize,
        llrr: [(Usize1, Usize1); q],
    };
    let mut xxx = vec![vec![0; n + 1]; k];
    for j in 0..k {
        for (i, a) in aa.iter().copied().enumerate() {
            xxx[j][i + 1] = xxx[j][i] + if i % k == j { a } else { 0 };
        }
    }
    let get = |j: usize, l: usize, r: usize| -> isize { xxx[j][r] - xxx[j][l] };
    for (l, r) in llrr {
        let mut tf = true;
        let tmp = get(0, l, r + 1);
        for j in 1..k {
            tf &= tmp == get(j, l, r + 1);
        }
        let rs = ["No", "Yes"][tf as usize];
        println!("{rs}");
    }
}
