#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library::ModInt998244353;
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    };
    // A < C
    // A < D
    // B < D

    // A < D を事前に配置する
    // Dの前後 D+1 個のスポットにCを C 個配置する
    // Dの前のスポット（Dの前に置かれたCの数だけスポットが増える）にBを配置する
    // let mut rs = ModInt998244353::default();
    // println!("{rs}");
}
