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
        aa: [Usize1; n],
        q: usize,
        llrr: [(usize, usize); q],
    };
    // Mo's algorithm
    let size = n.sqrt();
    let mut qqq = vec![vec![]; size + 2];
    for (i, (l, r)) in llrr.into_iter().enumerate() {
        let l = l - 1;
        qqq[l / size].push((r, l, i));
    }
    let mut rrss = vec![0; q];
    for mut qq in qqq {
        if qq.is_empty() {
            continue;
        }
        qq.sort_unstable();
        let mut l = qq[0].1;
        let mut r = l;
        let mut vv = vec![0; n];
        let mut cnt = 0;
        for (qr, ql, qi) in qq {
            while r < qr {
                vv[aa[r]] += 1;
                if vv[aa[r]].is_even() {
                    cnt += 1;
                }
                r += 1;
            }
            while l < ql {
                vv[aa[l]] -= 1;
                if vv[aa[l]].is_odd() {
                    cnt -= 1;
                }
                l += 1;
            }
            while ql < l {
                l -= 1;
                vv[aa[l]] += 1;
                if vv[aa[l]].is_even() {
                    cnt += 1;
                }
            }
            rrss[qi] = cnt;
        }
    }
    for rs in rrss {
        println!("{rs}");
    }
}
