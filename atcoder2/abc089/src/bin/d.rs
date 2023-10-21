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
        h: usize,
        w: usize,
        d: usize,
        aaa: [[usize; w]; h],
        q: usize,
        llrr: [(usize, usize); q],
    };
    // a % d ごとの累積和
    let mut sss = vec![];
    {
        let mut hm = HashMap::new();
        for (i, aa) in aaa.into_iter().enumerate() {
            for (j, a) in aa.into_iter().enumerate() {
                hm.insert(a, (i, j));
            }
        }
        // a = d * k + m
        for m in 0..d {
            let kl = usize::from(m == 0);
            let kr = (h * w - m) / d;
            let ss = chain!(std::iter::once(kl), kl..=kr)
                .map(|k| *hm.get(&(d * k + m)).unwrap())
                .tuple_windows()
                .map(|((ix, iy), (jx, jy))| ix.abs_diff(jx) + iy.abs_diff(jy))
                .cumsum::<usize>()
                .collect_vec();
            // eprintln!("{m} [{}]", ss.iter().join(","));
            sss.push(ss);
        }
    }
    for (l, r) in llrr {
        let m = l % d;
        let kl = (l - m) / d - usize::from(m == 0);
        let kr = (r - m) / d - usize::from(m == 0);
        // eprintln!("{m}[{l}-{r}] {kl}-{kr}");
        let rs = sss[m][kr] - sss[m][kl];
        // eprintln!("{m}[{l}-{r}] {kl}:{} {kr}:{}", sss[m][kl], sss[m][kr]);
        println!("{rs}");
    }
}
