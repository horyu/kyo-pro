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
        n: u128,
    };
    // xxx - yyy = n
    // xxx - n = yyy
    for x in (n.nth_root(3) + 1)..(4e7 as u128) {
        let yyy = x.pow(3u32) - n;
        let y = yyy.nth_root(3);
        if y.pow(3u32) == yyy {
            println!("{x} {y}");
            return;
        }
    }

    // (i + 1)^3 - i^3 <= 1e18 となる最大のi
    // let i = 577_350_268u128; // 5.7e8
    // dbg!(i, i.ilog10());
    // let diff = (i + 1).pow(3u32) - i.pow(3u32);
    // dbg!(diff, diff.ilog10());

    println!("-1");
}
