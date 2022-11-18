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
        xx: Bytes,
    };
    let xx = xx
        .into_iter()
        .rev()
        .map(|x| (x - b'0') as usize)
        .collect_vec();
    let mut vv = vec![];
    let mut sum = xx.iter().sum::<usize>();
    let mut amari = 0;
    for x in xx {
        let y = sum + amari;
        vv.push(y % 10);
        amari = y / 10;
        sum -= x;
    }
    while 0 < amari {
        vv.push(amari % 10);
        amari /= 10;
    }
    if let Some(&0) = vv.last() {
        vv.pop();
    }
    println!("{}", vv.into_iter().rev().join(""));
}
