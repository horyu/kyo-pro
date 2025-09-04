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
        k: u128,
    };
    let mut vv = vec![];
    for i in 1..=k.sqrt() {
        if k % i == 0 {
            vv.push(i);
            if k / i != i {
                vv.push(k / i);
            }
        }
    }
    vv.sort_unstable();
    let len = vv.len();
    // dbg!(len);

    let mut rs = 0usize;
    for ai in 0..len {
        let a = vv[ai];
        for bi in ai..len {
            let b = vv[bi];
            let c = k / (a * b);
            if b <= c && a * b * c == k {
                // eprintln!("{} {} {}", a, b, c);
                rs += 1;
            }
        }
    }

    println!("{rs}");
}
