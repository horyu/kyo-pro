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
    // 1の位から何回1があるか集計する
    //   1: 1 11 21 .. 91 101 111 ..
    //  1*: 10..19 110..119 210..219
    // 1**: 100..199 1100..1199
    let mut rs = 0usize;
    for i in 1u32..=(n.ilog10() + 1) {
        // 10^i のループの個数
        rs += dbg!(n / 10usize.pow(i) * 10usize.pow(i - 1));
        // 最後のループの個数
        rs += dbg!((n % 10usize.pow(i))
            .min(10usize.pow(i - 1) * 2 - 1)
            .saturating_sub(10usize.pow(i - 1) - 1));
    }
    println!("{rs}");
}
