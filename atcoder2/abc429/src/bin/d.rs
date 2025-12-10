#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[cfg(not(debug_assertions))]
macro_rules! eprintln {
    ($($tt:tt)*) => {};
}

fn main() {
    input! {
        n: usize,
        m: usize,
        c: usize,
        mut aa: [usize; n],
    };
    aa.sort_unstable();
    aa.extend(chain!(
        aa.clone().into_iter().map(|a| a + m),
        aa.clone().into_iter().map(|a| a + 2 * m)
    ));
    let ccpp = aa.into_iter().dedup_with_count().collect_vec();
    let len = ccpp.len();
    let sspp = ccpp
        .iter()
        .scan(0usize, |acc, &(cnt, pos)| {
            *acc += cnt;
            Some((*acc, pos))
        })
        .collect_vec();
    eprintln!("{ccpp:?}");
    eprintln!("{sspp:?}");
    let mut rs = 0usize;
    let mut pre_i = 0usize;
    for (i, (si, pi)) in sspp.iter().copied().enumerate().take(len) {
        if pi == 0 {
            continue;
        }
        let j = i + sspp[i..].partition_point(|(sj, pj)| sj - si < c);
        // dbg!(si, sspp[j]);
        eprintln!(
            "[{i}: ({si},{pi})] {:?} -> diff:{} rs:{rs}",
            sspp[j],
            sspp[j].0 - si
        );
        rs += (sspp[j].0 - si) * (pi - pre_i);
        pre_i = pi;
    }
    println!("{rs}");
}
