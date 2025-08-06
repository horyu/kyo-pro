#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        t: usize,
        qq: [[isize; 3]; t],
    };
    for mut xx in qq {
        // -2, +0, +2 を自由に複数回割り当てて同じ値にできるか
        if !xx.iter().map(|x| x % 2).all_equal() {
            println!("-1");
            continue;
        }
        xx.sort_unstable();
        let (d1, d2) = (xx[1] - xx[0], xx[2] - xx[1]);
        let (d_sml, d_big) = if d1 < d2 { (d1, d2) } else { (d2, d1) };
        // let rs = if d_sml == d_big {
        //     d_sml / 2
        // } else if d_sml == 0 {
        //     d_big / 3
        // } else {
        //     d_sml / 2 + (d_big - d_sml) / 3
        // };
        let rs = d_sml / 2 + (d_big - d_sml) / 3;
        println!("{rs}");
    }
}
