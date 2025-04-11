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
        x: usize,
        uudd: [(usize, usize); n],
    };
    let (u_min, u_max, ud_min, ud_max) =
        uudd.iter()
            .copied()
            .fold((!0, 0, !0, 0), |(min_u, max_u, min_ud, max_ud), (u, d)| {
                (
                    min_u.min(u),
                    max_u.max(u),
                    min_ud.min(u + d),
                    max_ud.max(u + d),
                )
            });
    // let mut ok = 0;
    // let mut ng = ud_min + 1;
    // while 1 < ng - ok {
    //     let mid = ok.midpoint(ng);
    //     let mut sml = mid.saturating_sub(uudd[0].1);
    //     let mut big = mid.min(uudd[0].0);
    // }

    // println!("{rs}");
}
