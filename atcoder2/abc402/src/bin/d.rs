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
        m: usize,
        aabb: [(Usize1, Usize1); m],
    };
    let mut counter = counter::Counter::<_>::new();
    let mut rs = m * m.saturating_sub(1) / 2;
    // 円周上の直線の組が平行であるのは 同じ傾きであるとき == 垂直線が一致する
    // 隣接した点の中点を取れるように頂点の数を2倍にして考える
    let nn = n * 2;
    for (a, b) in aabb {
        let mid = (a * 2 + b * 2) / 2;
        let x = (mid % nn).min((mid + n) % nn);
        rs -= counter[&x];
        counter[&x] += 1;
    }
    println!("{rs}");
}
