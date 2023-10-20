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
        cc: Chars
    };
    let n = cc.len();
    let ttff = cc.iter().copied().map(|c| c == 'o').collect_vec();
    let mut rs = u32::MAX;
    for bits in 0u32..(1 << n) {
        let mut vv = vec![false; n];
        for i in 0..n {
            if bits & (1 << i) != 0 {
                for (j, tf) in ttff.iter().copied().enumerate() {
                    if tf {
                        vv[(i + j) % n] = tf;
                    }
                }
            }
        }
        if vv.iter().all(|&v| v) {
            rs = rs.min(bits.count_ones());
        }
    }
    println!("{rs}");
}

#[allow(dead_code)]
fn main2() {
    input! {
        cc: Chars
    };
    let n = cc.len();
    let ttff = cc.iter().copied().map(|c| c == 'o').collect_vec();
    for size in 1..=n {
        for dd in (0..n).permutations(size) {
            let mut vv = vec![false; n];
            for d in dd {
                for (i, tf) in ttff.iter().copied().enumerate() {
                    if tf {
                        vv[(i + d) % n] = tf;
                    }
                }
            }
            if vv.iter().all(|&v| v) {
                println!("{size}");
                return;
            }
        }
    }
}
