#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        s: Bytes,
        q: usize,
        ttkk: [(usize, Usize1); q],
    };
    // C   B   A
    // A B C A B C
    // BCCAABBCCAAB
    let s = s.into_iter().map(|c| (c - b'A') as usize).collect_vec();
    for (mut t, mut k) in ttkk {
        let mut diff = 0;
        while 0 < t && 0 < k {
            if k.is_even() {
                diff += 1;
            } else {
                diff += 2;
            }
            k /= 2;
            t -= 1;
        }
        let rs = ['A', 'B', 'C'][(s[k] + t + diff) % 3];
        println!("{rs}");
    }
}
