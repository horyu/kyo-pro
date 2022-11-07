#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use ac_library_rs::ModInt998244353;
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    };
    let mut vv = vec![ModInt998244353::from(0); n];
    vv[0] += 1;
    let mut bunsi = ModInt998244353::from(0);
    for kk in 0..k {
        bunsi *= m;
        let mut new_vv = vec![ModInt998244353::from(0); n];
        for (i, v) in vv.into_iter().enumerate() {
            for mm in 1..=m {
                let ii = i + mm;
                match ii.cmp(&n) {
                    std::cmp::Ordering::Less => {
                        new_vv[ii] += v;
                    }
                    std::cmp::Ordering::Equal => {
                        bunsi += v;
                    }
                    std::cmp::Ordering::Greater => {
                        new_vv[n - (ii - n)] += v;
                    }
                }
            }
        }
        vv = new_vv;
    }
    // dbg!(vv, &bunsi);
    let bunbo = ModInt998244353::from(m).pow(k as u64);
    let rs = bunsi / bunbo;
    println!("{rs}");
}
