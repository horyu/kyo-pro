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
        n: usize,
    };
    for i in (1..=n.nth_root(3)).rev() {
        let x = i.pow(3);
        let mut tmp = x;
        let mut vv = vec![];
        while 0 < tmp {
            vv.push((tmp % 10) as u8);
            tmp /= 10;
        }
        let ww = vv.iter().copied().rev().collect_vec();
        if vv == ww {
            println!("{x}");
            return;
        }
    }
}
