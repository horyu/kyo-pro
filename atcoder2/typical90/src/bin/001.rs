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
        l: usize,
        k: usize,
        aa: [usize; n],
    };
    let mut ok = 1;
    let mut ng = l + 1;
    while 1 < ng - ok {
        let m = (ok + ng) / 2;
        let mut cnt = 0;
        let mut pre_a = 0;
        for a in aa.iter().copied() {
            if m <= a - pre_a && m <= l - a {
                cnt += 1;
                pre_a = a;
            }
        }
        // eprintln!("{cnt} {pre_a} {ok} {m} {ng}");
        if k <= cnt {
            ok = m;
        } else {
            ng = m;
        }
    }
    let rs = ok;
    println!("{rs}");
}
