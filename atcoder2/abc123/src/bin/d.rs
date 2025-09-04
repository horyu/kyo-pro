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
        x: usize,
        y: usize,
        z: usize,
        k: usize,
        mut aa: [usize; x],
        mut bb: [usize; y],
        mut cc: [usize; z],
    };
    aa.sort_unstable_by_key(|&a| R(a));
    bb.sort_unstable_by_key(|&b| R(b));
    cc.sort_unstable_by_key(|&c| R(c));
    let mut bh = BinaryHeap::new();
    let mut pushed = HashSet::new();
    bh.push((aa[0] + bb[0] + cc[0], 0, 0, 0));
    pushed.insert((0, 0, 0));

    let mut rrss = Vec::new();
    while let Some((s, ia, ib, ic)) = bh.pop() {
        rrss.push(s);
        if rrss.len() == k {
            break;
        }
        for &(da, db, dc) in &[(1, 0, 0), (0, 1, 0), (0, 0, 1)] {
            let ja = ia + da;
            let jb = ib + db;
            let jc = ic + dc;
            if ja < x && jb < y && jc < z && pushed.insert((ja, jb, jc)) {
                bh.push((aa[ja] + bb[jb] + cc[jc], ja, jb, jc));
            }
        }
    }
    let rs = rrss.iter().join("\n");
    println!("{rs}");
}
