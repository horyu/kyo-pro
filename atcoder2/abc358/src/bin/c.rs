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
        m: usize,
        ss: [Chars; n],
    };
    let kk = ss
        .into_iter()
        .map(|s| {
            s.into_iter()
                .fold(0, |bit, c| (bit << 1) | (c == 'o') as usize)
        })
        .collect_vec();
    for size in 1..=n {
        for bits in kk.iter().copied().combinations(size) {
            if (1 << m) - 1 == bits.iter().copied().fold(0, |acc, x| acc | x) {
                println!("{size}");
                return;
            }
        }
    }
}
