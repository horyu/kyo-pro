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
        pp: [usize; n],
    };
    // p[i] < p[i+1] > p[i+2] を満たすi
    let mut xx = BTreeSet::new();
    // p[i] > p[i+1] < p[i+2] を満たすi
    let mut yy = BTreeSet::new();
    for (i, (p1, p2, p3)) in pp.iter().copied().tuple_windows().enumerate() {
        if p1 < p2 && p2 > p3 {
            xx.insert(i);
        } else if p1 > p2 && p2 < p3 {
            yy.insert(i);
        }
    }
    // eprintln!("xx:{xx:?}\nyy:{yy:?}");
    let mut rs = 0;
    for (i, (p1, p2)) in pp.iter().copied().tuple_windows().enumerate() {
        if p2 <= p1 {
            continue;
        }
        let xxjj = xx.range(i..).copied().take(2).collect_vec();
        let yyjj = yy.range(i..).copied().take(2).collect_vec();
        let jj = chain!(&xxjj, &yyjj)
            .copied()
            .chain(std::iter::once(n - 2))
            .sorted_unstable()
            .collect_vec();
        if !xxjj.is_empty()
            && !yyjj.is_empty()
            && (jj[0] == xxjj[0] || jj[0] == yyjj[0])
            && (jj[1] == xxjj[0] || jj[1] == yyjj[0])
        {
            rs += jj[2] - jj[1];
            // eprintln!("{i} {jj:?} {xxjj:?} {yyjj:?} +{}", jj[2] - jj[1]);
            // let k = jj[2] + 1;
            // eprintln!("{i}-{k}: {}", pp[i..=k].iter().join(","));
        }
    }
    println!("{rs}");
}
