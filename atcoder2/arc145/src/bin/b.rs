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
    if a <= b {
        // 初手が行えるようになったら全勝
        println!("{}", n.saturating_sub(a - 1));
        return;
    }
    let mut rs = 0usize;
    // (k*a)..(k*a+b) の間だけ勝ち
    let block = n / a;
    rs += block.saturating_sub(1) * b;
    if 0 < block {
        // 最後のブロックの途中まで処理
        rs += (n - a * block + 1).min(b);
    }
    println!("{rs}");
}
