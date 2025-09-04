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
        q: usize,
        cc: [Usize1; n],
        llrr: [(usize, usize); q],
    };
    // Mo's algorithm
    // https://kanpurin.hatenablog.com/entry/2021/04/09/001439
    let size = n.sqrt();
    let ii = (0..q).sorted_unstable_by(|&a, &b| {
        let (al, ar) = llrr[a];
        let (bl, br) = llrr[b];
        if al / size != bl / size {
            al.cmp(&bl)
        } else if al / size % 2 == 1 {
            br.cmp(&ar)
        } else {
            ar.cmp(&br)
        }
    });
    let mut rrss = vec![0; q];
    let mut l = 0;
    let mut r = 0;
    let mut vv = vec![0i32; n];
    let mut cnt = 0;
    for i in ii {
        let (ql, qr) = llrr[i];
        let ql = ql - 1;
        while qr < r {
            r -= 1;
            vv[cc[r]] -= 1;
            if vv[cc[r]] == 0 {
                cnt -= 1;
            }
        }
        while r < qr {
            vv[cc[r]] += 1;
            if vv[cc[r]] == 1 {
                cnt += 1;
            }
            r += 1;
        }
        while l < ql {
            vv[cc[l]] -= 1;
            if vv[cc[l]] == 0 {
                cnt -= 1;
            }
            l += 1;
        }
        while ql < l {
            l -= 1;
            vv[cc[l]] += 1;
            if vv[cc[l]] == 1 {
                cnt += 1;
            }
        }
        rrss[i] = cnt;
    }
    for rs in rrss {
        println!("{rs}");
    }
}

#[allow(dead_code)]
fn main2() {
    input! {
        n: usize,
        q: usize,
        cc: [Usize1; n],
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
                vv[cc[r]] += 1;
                if vv[cc[r]] == 1 {
                    cnt += 1;
                }
                r += 1;
            }
            while l < ql {
                vv[cc[l]] -= 1;
                if vv[cc[l]] == 0 {
                    cnt -= 1;
                }
                l += 1;
            }
            while ql < l {
                l -= 1;
                vv[cc[l]] += 1;
                if vv[cc[l]] == 1 {
                    cnt += 1;
                }
            }
            rrss[qi] = cnt;
        }
    }
    for rs in rrss {
        println!("{}", rs);
    }
}
