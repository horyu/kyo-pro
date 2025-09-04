#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
        aa: [usize; n],
        bb: [usize; m],
        ccdd: [(Usize1, Usize1); l],
    };
    let ngs = HashSet::<_>::from_iter(ccdd);
    let ii = (0..n)
        .sorted_unstable_by_key(|&i| aa[i])
        .rev()
        .collect_vec();
    let jj = (0..m)
        .sorted_unstable_by_key(|&j| bb[j])
        .rev()
        .collect_vec();
    // eprintln!("{:?}", ii);
    // eprintln!("{:?}", jj);
    let mut bh = BinaryHeap::new();
    bh.push((aa[ii[0]] + bb[jj[0]], 0, 0));
    let mut pushed = HashSet::new();
    pushed.insert((0, 0));
    while let Some((rs, qi, qj)) = bh.pop() {
        // eprintln!("{rs} {qi} {qj}");
        if ngs.contains(&(ii[qi], jj[qj])) {
            let pi = qi + 1;
            if pi < n && pushed.insert((pi, qj)) {
                bh.push((aa[ii[pi]] + bb[jj[qj]], pi, qj));
            }
            let pj = qj + 1;
            if pj < m && pushed.insert((qi, pj)) {
                bh.push((aa[ii[qi]] + bb[jj[pj]], qi, pj));
            }
            continue;
        }
        println!("{rs}");
        return;
    }
}
