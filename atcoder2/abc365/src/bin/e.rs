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
        aa: [usize; n],
    };
    let mut rs = 0usize;
    for d in 0..=(1e8 as usize).ilog2() {
        let bb = aa.iter().map(|a| a >> d & 1).collect_vec();
        // cnt[i] = 手前まで見たときに累積XORがiであるものの個数
        let mut cnt = [0; 2];
        for b in bb.iter().copied() {
            rs += cnt[1 - b] << d;
            if b == 0 {
                cnt[0] += 1;
            } else {
                cnt = [cnt[1], cnt[0] + 1];
            }
        }
    }
    println!("{rs}");
}
