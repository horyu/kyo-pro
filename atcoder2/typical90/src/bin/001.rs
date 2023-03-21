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
        l: usize,
        k: usize,
        aa: [usize; n],
    };
    let mut ok = 1;
    let mut ng = l + 1;
    while 1 < ng - ok {
        let m = (ng + ok) / 2;
        let mut cnt = 0;
        let mut pre = 0;
        for a in aa.iter().copied() {
            if m <= (a - pre).min(l - a) {
                cnt += 1;
                pre = a;
            }
        }
        if cnt < k {
            ng = m;
        } else {
            ok = m;
        }
    }
    let rs = ok;
    println!("{rs}");
}
