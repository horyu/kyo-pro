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
        k: usize,
        mut aa: [usize; n],
        mut ff: [usize; n],
    };
    if aa.iter().sum::<usize>() <= k {
        println!("0");
        return;
    }
    aa.sort_unstable();
    ff.sort_unstable();
    ff.reverse();

    let mut ok = 1e13 as usize;
    let mut ng = 0;
    let check = |m: usize| -> bool {
        let mut kk = k;
        for (a, f) in izip!(aa.iter().copied(), ff.iter().copied()) {
            if m < a * f {
                let cost = (a * f - m).div_ceil(f);
                if cost <= kk {
                    kk -= cost;
                } else {
                    return false;
                }
            }
        }
        true
    };
    while 1 < ok - ng {
        let m = (ok + ng) / 2;
        if check(m) {
            ok = m;
        } else {
            ng = m;
        }
    }
    let rs = ok;
    println!("{rs}");
}
