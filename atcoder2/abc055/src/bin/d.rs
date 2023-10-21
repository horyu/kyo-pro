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
        s: Chars,
    };
    let s = s.into_iter().map(|c| c == 'o').collect_vec();
    // true: 羊、false: 狼
    for mut vv in (0..2).map(|_| [false, true]).multi_cartesian_product() {
        for i in 0..(n - 2) {
            // 羊?, 同じ?
            let next = match (vv[i + 1], s[i + 1]) {
                (true, true) => vv[i],
                (true, false) => !vv[i],
                (false, true) => !vv[i],
                (false, false) => vv[i],
            };
            vv.push(next);
        }
        if (0..n).all(|i| {
            let pre = vv[(n + i - 1) % n];
            let next = vv[(n + i + 1) % n];
            match (vv[i], s[i]) {
                (true, true) => pre == next,
                (true, false) => pre != next,
                (false, true) => pre != next,
                (false, false) => pre == next,
            }
        }) {
            let rs = vv.into_iter().map(|v| ['W', 'S'][v as usize]).join("");
            println!("{rs}");
            return;
        }
    }

    println!("-1");
}
