#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};


fn main() {
    input! {
        n: usize,
        mut pp: [Usize1; n],
    };
    let mut rs = vec![];
    for l in 0..(n - 1) {
        if pp[l] == l {
            continue;
        }
        let mut ll = pp.iter().position(|&p| p == l).unwrap();
        if l.is_even() == ll.is_even() {
            continue;
        }
        let mut r = l + 1;
        let mut rr = loop {
            let rr = pp.iter().position(|&p| p == r).unwrap();
            if r.is_even() == rr.is_even() {
                r += 2;
                continue;
            }
            break rr;
        };
        if ll + 1 <= rr {
            while ll + 1 != rr {
                pp.swap(rr - 2, rr);
                rs.push(('B', rr - 1));
                rr -= 2;
            }
            rs.push(('A', rr));
            pp.swap(ll, rr);
        } else {
            while rr + 1 != ll {
                pp.swap(ll - 2, ll);
                rs.push(('B', ll - 1));
                ll -= 2;
            }
            rs.push(('A', ll));
            pp.swap(ll, rr);
        }
    }
    for oe in [0, 1] {
        let mut l = oe;
        let mut r = (oe..n).step_by(2).last().unwrap();
        while l < r {
            let mut ll = pp.iter().position(|&p| p == l).unwrap();
            let mut rr = pp.iter().position(|&p| p == r).unwrap();
            let ld = ll.abs_diff(l);
            let rd = rr.abs_diff(r);
            if ld <= rd {
                while l != ll {
                    pp.swap(ll - 2, ll);
                    rs.push(('B', ll - 1));
                    ll -= 2;
                }
                l += 2;
            } else {
                while r != rr {
                    pp.swap(rr, rr + 2);
                    rs.push(('B', rr + 1));
                    rr += 2;
                }
                r -= 2;
            }
        }
    }
    println!("{}", rs.len());
    for (c, i) in rs {
        println!("{c} {i}");
    }
}
