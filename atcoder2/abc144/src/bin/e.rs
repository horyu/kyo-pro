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
        k: isize,
        mut aa: [isize; n],
        mut ff: [isize; n],
    };
    aa.sort_unstable();
    aa.reverse();
    ff.sort_unstable();
    let mut ok = isize::MAX >> 1;
    let mut ng = -1;
    while 1 < ok - ng {
        let mid = (ok + ng) / 2;
        let mut cnt = 0;
        for (&a, &f) in izip!(&aa, &ff) {
            let af = a * f;
            if mid < af {
                // (a - x) * f <= mid
                // (fa - mid) / f <= x
                cnt += (af - mid).div_ceil(f);
            }
        }
        if cnt <= k {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    let rs = ok;
    println!("{rs}");
}
