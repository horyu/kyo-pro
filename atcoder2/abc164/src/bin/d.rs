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
        s: Bytes,
    };
    let n = s.len();
    let s = s.into_iter().map(|b| (b - b'0') as usize).collect_vec();
    let mut cc = vec![0usize; 2019];
    cc[0] = 1;
    let mut x = 0;
    let mut mul = 1;
    for b in s.iter().copied().rev() {
        x = (x + b * mul) % 2019;
        cc[x] += 1;
        mul = mul * 10 % 2019;
    }
    // eprintln!();
    // for i in 0..2019 {
    //     if cc[i] != 0 {
    //         eprintln!("{i} {}", cc[i]);
    //     }
    // }
    let mut rs = 0;
    for c in cc {
        rs += (c * c.saturating_sub(1)) / 2;
    }
    println!("{rs}");
}

#[allow(dead_code)]
fn main2() {
    input! {
        s: Bytes,
    };
    const N: usize = 2019;
    let mut vv = [0usize; N];
    let mut rs = 0;
    for b in s {
        let b = (b - b'0') as usize;
        let mut new_vv = [0; N];
        for (i, v) in vv.iter().copied().enumerate() {
            new_vv[(i * 10 + b) % N] += v;
        }
        new_vv[b] += 1;
        vv = new_vv;
        // let tmp = vv.iter().copied().enumerate().filter(|iv| 0 < iv.1)
        //     .map(|ic| format!("{ic:?}"))
        //     .join(" ");
        // eprintln!("{tmp}");
        rs += vv[0];
    }
    println!("{rs}");
}
