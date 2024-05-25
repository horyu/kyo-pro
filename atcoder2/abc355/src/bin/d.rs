#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        llrr: [(usize, usize); n],
    };
    // 座標圧縮
    let x2i = llrr
        .iter()
        .copied()
        .flat_map(|(l, r)| [l, r])
        .sorted_unstable()
        .dedup()
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect::<HashMap<_, _>>();
    let aabb = llrr
        .iter()
        .copied()
        .map(|(l, r)| (x2i[&l], x2i[&r]))
        .sorted_unstable_by_key(|&(a, b)| (b, R(a)))
        .collect_vec();
    let mut ft = ac_library::FenwickTree::new(x2i.len() + 1, 0usize);
    // 区間の重複を数える
    let mut rs = 0usize;
    for (a, b) in aabb {
        // eprintln!("{a} {b} {:?}", (0..=x2i.len()).map(|i| ft.sum(i..=i)).collect_vec());
        rs += ft.sum(a..=b);
        ft.add(b, 1);
    }
    println!("{rs}");
}
