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
    };
    for _ in 0..t {
        input! {
            n: usize,
            mut aa: [isize; n],
        };
        aa.sort_unstable_by_key(|&a| a.abs());
        let mut tf = aa
            .iter()
            .copied()
            .tuple_windows()
            .all(|(x, y, z)| x * z == y * y);
        // r = -1 パターン
        if !tf {
            let vv = aa.iter().copied().counts().into_iter().collect_vec();
            if vv.len() == 2 {
                let (x, y) = (vv[0], vv[1]);
                tf |= x.0.abs() == y.0.abs() && x.1.abs_diff(y.1) <= 1;
            }
        }
        let rs = ["No", "Yes"][tf as usize];
        println!("{rs}");
    }
}
