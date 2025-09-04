#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
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
        mut cc: [usize; n],
    };
    // https://img.atcoder.jp/arc024/editorial.pdf#page=13&zoom=auto,-16,595
    if cc.iter().all_equal() {
        println!("-1");
        return;
    }
    let zure = cc.iter().position(|&c| c != cc[0]).unwrap();
    cc.rotate_left(zure);
    let mut rs = 0;
    for (_, g) in cc.into_iter().group_by(|&c| c).into_iter() {
        rs = rs.max((g.into_iter().count() - 1) / 2 + 1);
    }
    println!("{rs}");
}
