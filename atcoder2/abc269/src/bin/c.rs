#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        x: i64,
    };
    let mut vv = vec![];
    let mut i = (1 << 60) - 1;
    while 0 <= i {
        i &= x;
        vv.push(i);
        i -= 1;
    }
    let rs = vv.into_iter().rev().join("\n");
    println!("{rs}");
}

#[allow(dead_code)]
fn main2() {
    input! {
        mut x: usize,
    };
    if x == 0 {
        println!("0");
        return;
    }
    let mut vvv = vec![];
    while x != 0 {
        if x & 1 == 1 {
            vvv.push(vec![0, 1]);
        } else {
            vvv.push(vec![0]);
        }
        x >>= 1;
    }
    for ww in vvv.into_iter().rev().multi_cartesian_product() {
        let rs = ww.into_iter().fold(0usize, |acc, w| w + (acc << 1));
        println!("{rs}");
    }
}
