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
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
        f: usize,
    };
    const MAX: usize = 3000;
    // vv: 水のみ　ww: 砂糖のみ
    let mut vv = vec![false; MAX + 1];
    vv[0] = true;
    let mut ww = vv.clone();
    for i in 0..MAX {
        if vv[i] {
            let a_next = i + 100 * a;
            if a_next <= MAX {
                vv[a_next] = true;
            }
            let b_next = i + 100 * b;
            if b_next <= MAX {
                vv[b_next] = true;
            }
        }
        if ww[i] {
            let c_next = i + c;
            if c_next <= MAX {
                ww[c_next] = true;
            }
            let d_next = i + d;
            if d_next <= MAX {
                ww[d_next] = true;
            }
        }
    }
    // v: 水　w: 砂糖
    let mut v = 100 * a;
    let mut w = 0;
    for i in (100 * a)..MAX {
        if !vv[i] {
            continue;
        }
        for j in 1..MAX {
            if f < i + j {
                break;
            }
            if !ww[j] {
                continue;
            }
            // 100 : e = i : j
            // 100j/i <= e
            // 100j <= e*i
            if 100 * j <= e * i {
                // 100w/(v+w) < 100j/(i + j)
                // 100w * (i+j) < 100j * (v+w)
                if w * (i + j) < j * (v + w) {
                    v = i;
                    w = j;
                }
            }
        }
    }
    println!("{} {}", v + w, w);
}
