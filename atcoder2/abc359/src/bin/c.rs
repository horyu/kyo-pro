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
        sx: usize,
        sy: usize,
        tx: usize,
        ty: usize,
    };
    let dy = sy.abs_diff(ty);
    let mut rs = dy;
    // 縦移動の回数だけ左右移動できる
    let s_left = sx % 2 == sy % 2;

    let (l, r) = if s_left {
        (sx.saturating_sub(dy), 1 + sx + dy)
    } else {
        (sx.saturating_sub(1 + dy), sx + dy)
    };
    // dbg!(rs, l, r);
    if tx < l {
        rs += tx.abs_diff(l).div_ceil(2);
    } else if r < tx {
        rs += tx.abs_diff(r).div_ceil(2);
    }
    println!("{rs}");
}
