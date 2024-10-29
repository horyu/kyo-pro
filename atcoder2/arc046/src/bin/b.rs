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
        a: usize,
        b: usize,
    };
    // https://img.atcoder.jp/arc046/editorial.pdf

    // n <= a の場合
    //   先手: 全部取って勝ち
    // a < b の場合(2手以上続く)
    //   先手: ak + 1 個になるように取っていく
    //   後手: bk + 1 個になるように取っていく

    let tf = if n <= a {
        true
    } else if a == b {
        n % (a + 1) != 0
    } else {
        b < a
    };
    let rs = ["Aoki", "Takahashi"][tf as usize];
    println!("{rs}");
}
