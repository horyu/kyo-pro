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
        q: usize,
        mut aa: [isize; n],
        bbkk: [(isize, usize); q],
    };
    aa.sort_unstable();
    // eprintln!("{aa:?}");
    for (b, mut k) in bbkk {
        // let orik = k;
        let mut l = aa.partition_point(|&a| a < b);
        let mut r = aa.partition_point(|&a| a <= b);
        k = k.saturating_sub(r - l);
        while 0 < k {
            if l == 0 {
                r += k;
                k = 0;
            } else if r == n {
                l -= k;
                k = 0;
            } else {
                let m = k.div_ceil(2);
                let ll = l.saturating_sub(m);
                let rr = (r + m).min(n);
                if b - aa[ll] <= aa[rr - 1] - b {
                    k -= l - ll;
                    l = ll;
                } else {
                    k -= rr - r;
                    r = rr;
                }
            }
        }
        // dbg!(l..r);
        // assert!(orik <=  r - l);
        let rs = (b - aa[l]).max(aa[r - 1] - b);
        println!("{rs}");
    }
}
