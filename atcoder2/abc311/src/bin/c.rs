#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
    };
    let mut uf = UnionFind::new(n);
    for (i, a) in aa.iter().copied().enumerate() {
        if uf.union(i, a) {
            let mut pushed = HashSet::new();
            let mut vv = VecDeque::from([i]);
            let mut next = a;
            while pushed.insert(next) {
                vv.push_back(next);
                next = aa[next];
            }
            while let Some(&front) = vv.front() {
                if front == next {
                    break;
                }
                vv.pop_front();
            }
            println!("{}\n{}", vv.len(), vv.into_iter().map(|v| v + 1).join(" "));
            return;
        }
    }
}
