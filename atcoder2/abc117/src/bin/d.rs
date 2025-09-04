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
        k: usize,
        aa: [usize; n],
    };
    let mut bb = [0; 40];
    for mut a in aa.iter().copied() {
        for i in 0..40 {
            bb[i] += a & 1;
            a >>= 1;
        }
    }
    // 5 0~2 true 3~5 false
    // 4 0~1 true 2~4 false
    // 反転したほうが値が大きくなるビット
    let ttff = bb.iter().copied().map(|b| b < n.div_ceil(2)).collect_vec();
    // eprintln!("{bb:?}");
    // eprintln!("{ttff:?}");
    // dbg!(k.leading_zeros());
    let mut x = 0;
    // eprintln!("{:?}", (0..(64 - k.leading_zeros())).rev().collect_vec());
    for i in (0..(64 - k.leading_zeros())).rev() {
        if ttff[i as usize] {
            let new_x = x | (1 << i);
            // eprintln!("[{i}] {new_x}");
            if new_x <= k {
                x = new_x;
            }
        }
    }
    // eprintln!("{x} {x:b}");
    let rs = aa.iter().copied().map(|a| a ^ x).sum::<usize>();
    println!("{rs}");
}
