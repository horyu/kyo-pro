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
        n: usize,
        aa: [usize; n],
        bb: [usize; n],
    };
    // let ihm: HashMap<usize, usize> = bb
    //     .iter()
    //     .copied()
    //     .sorted_unstable()
    //     .rev()
    //     .dedup()
    //     .enumerate()
    //     .collect();
    let bhm: HashMap<usize, usize> = bb
        .iter()
        .copied()
        .sorted_unstable()
        .rev() // 最大のbを0にマップする
        .dedup()
        .enumerate()
        .map(|(i, b)| (b, i))
        .collect();
    let bb = bb
        .into_iter()
        .map(|b| bhm.get(&b).copied().unwrap())
        .collect_vec();
    let mut ft = ac_library::FenwickTree::new(bhm.len(), 0usize);

    let aabb = izip!(aa, bb)
        .sorted_unstable()
        .collect_vec();
    let mut rs = 0usize;
    for ((a, b), g) in aabb.into_iter().group_by(|&ab| ab).into_iter() {
        let cnt = g.count();
        ft.add(b, cnt);
        rs += ft.sum(..=b) * cnt;
        // eprintln!(
        //     "{a} {b}({}) {}",
        //     ihm.get(&b).copied().unwrap(),
        //     ft.sum(..=b)
        // );
    }
    println!("{rs}");
}
