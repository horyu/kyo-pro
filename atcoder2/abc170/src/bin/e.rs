#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    ops::Not,
};

fn main() {
    input! {
        n: usize,
        q: usize,
        aabb: [(usize, Usize1); n],
        ccdd: [(Usize1, Usize1); q],
    };
    // 園i 園児j レートa
    let mut i_to_aajj = vec![BTreeSet::new(); 2 * 1e5 as usize];
    let mut j_to_ia = vec![(0, 0); n];
    for (j, (a, b)) in aabb.iter().copied().enumerate() {
        i_to_aajj[b].insert((a, j));
        j_to_ia[j] = (b, a);
    }
    // (a, j)
    let mut bts = i_to_aajj
        .iter()
        .enumerate()
        .filter_map(|(i, aajj)| aajj.iter().max().copied())
        .collect::<BTreeSet<_>>();
    for (c, d) in ccdd {
        let (i, a) = j_to_ia[c];

        i_to_aajj[i].remove(&(a, c));
        if bts.remove(&(a, c)) {
            if let Some(aj) = i_to_aajj[i].iter().max().copied() {
                bts.insert(aj);
            }
        }

        // btsから移動先のトップを削除して挿入し直す
        if let Some(aj) = i_to_aajj[d].iter().max().copied() {
            bts.remove(&aj);
        }
        i_to_aajj[d].insert((a, c));
        bts.insert(i_to_aajj[d].iter().max().copied().unwrap());

        j_to_ia[c].0 = d;

        let rs = bts.iter().min().copied().unwrap().0;
        println!("{rs}");
    }
}
