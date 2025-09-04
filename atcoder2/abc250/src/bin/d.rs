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
        n: u128,
    };
    let max = (n / 2).nth_root(3) as usize;
    let mut vv = vec![true; max + 2];
    for i in 0..=1 {
        vv[i] = false;
    }
    let mut pp = vec![];
    for i in 2..=max {
        if !vv[i] {
            continue;
        }
        pp.push(i as u128);
        for j in 0.. {
            let ij = i * j;
            if max < ij {
                break;
            }
            vv[ij] = false;
        }
    }
    // eprintln!("{}", pp.iter().join(" "));
    let mut rs = 0;
    for (i, a) in pp.iter().copied().enumerate() {
        let j = pp.partition_point(|&b| a.saturating_mul(b.saturating_pow(3)) <= n);
        if i < j {
            // eprintln!("{i}-{j}");
            rs += j - i - 1;
        }
    }
    println!("{rs}");
}
