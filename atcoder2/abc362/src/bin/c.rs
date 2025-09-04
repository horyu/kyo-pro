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
        llrr: [(isize, isize); n],
    };
    let (sum_l, sum_r) = llrr
        .iter()
        .copied()
        .fold((0, 0), |(sum_l, sum_r), (l, r)| (sum_l + l, sum_r + r));
    if (sum_l..=sum_r).contains(&0) {
        let mut vv = llrr.iter().map(|lr| lr.0).collect_vec();
        let mut diff = -sum_l;
        for (v, (l, r)) in izip!(&mut vv, llrr) {
            if diff == 0 {
                break;
            }
            let d = r - l;
            let d = if diff.abs() <= d.abs() { diff } else { d };
            *v += d;
            diff -= d;
        }
        println!("Yes");
        let rs = vv.into_iter().join(" ");
        println!("{rs}");
    } else {
        println!("No");
    }
}
