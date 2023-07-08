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
        k: isize,
        aabb: [(usize, isize); n],
    };
    let mut hs = HashSet::new();
    hs.insert(0);
    for (a, b) in aabb.iter().copied() {
        hs.insert(a);
    }
    let a2i = HashMap::<usize, usize>::from_iter(
        hs.into_iter()
            .sorted_unstable()
            .enumerate()
            .map(|(i, a)| (a, i)),
    );
    let mut ww = vec![0; a2i.len() + 1];
    for (a, b) in aabb.iter().copied() {
        let i = a2i.get(&a).copied().unwrap();
        ww[0] += b;
        ww[i] -= b;
    }
    let mut tmp = 0;
    for (i, w) in ww.into_iter().enumerate() {
        tmp += w;
        if tmp <= k {
            let rs = a2i
                .into_iter()
                .find_map(|(a, ori_i)| (i == ori_i).then_some(a))
                .unwrap()
                + 1;
            println!("{rs}");
            return;
        }
    }
}
