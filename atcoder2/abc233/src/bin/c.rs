#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        x: usize,
    };
    input! {
        l: usize,
        aa: [usize; l],
    };
    let mut hm = aa.into_iter().counts();
    for _ in 1..n {
        input! {
            l: usize,
            aa: [usize; l],
        };
        let mut new_hm = HashMap::new();
        for (ak, ac) in aa.into_iter().counts() {
            for (&k, &c) in &hm {
                if let Some(kk) = k.checked_mul(ak) {
                    if kk <= x {
                        *new_hm.entry(kk).or_insert(0) += ac * c;
                    }
                }
            }
        }
        hm = new_hm;
    }
    let rs = hm.get(&x).unwrap_or(&0);
    println!("{rs}");
}
