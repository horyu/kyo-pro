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
        hh: [Usize1; n],
    };
    let mut h2i = vec![!0; n];
    for (i, h) in hh.iter().copied().enumerate() {
        h2i[h] = i;
    }
    // 後ろから見ると単調現象（前から見ると単調増加）になる部分列の大きさ
    let mut cc = vec![1; n];
    let mut rrss = vec![0; n];
    let mut bts = BTreeSet::new();
    for i in (0..n).rev() {
        // eprintln!("{i} {rrss:?} {bts:?}");
        let hi = hh[i];
        if let Some(hj) = bts.range(hi..).next().copied() {
            let cj = cc[h2i[hj]];
            rrss[i] += cj;
            cc[i] += cj;
        }
        while let Some(h) = bts.range(..hi).next().copied() {
            bts.remove(&h);
            rrss[i] += 1;
        }
        bts.insert(hi);
    }
    // eprintln!("  {rrss:?} {bts:?}");
    let rs = rrss.iter().join(" ");
    println!("{rs}");
}
