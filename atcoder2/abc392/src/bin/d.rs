#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
    };
    // let mut aaa = vec![];
    let mut kk = vec![];
    let mut hhmm = vec![];
    for _ in 0..n {
        input! {k: usize, aa: [usize; k]};
        kk.push(k);
        hhmm.push(aa.into_iter().counts());
    }
    let mut rs = 0.0f64;
    for i in 0..n {
        let ik = kk[i];
        let ihm = &hhmm[i];
        for j in (i + 1)..n {
            let jk = kk[j];
            let jhm = &hhmm[j];
            let bunbo = ik * jk;
            let mut bunsi = 0;
            let (sml, big) = if ik < jk { (ihm, jhm) } else { (jhm, ihm) };
            for (k, v) in sml {
                bunsi += v * big.get(k).unwrap_or(&0);
            }
            rs = rs.max(bunsi as f64 / bunbo as f64);
        }
    }
    println!("{rs:0.15}");
}
