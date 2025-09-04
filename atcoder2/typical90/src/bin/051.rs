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
        p: usize,
        aa: [usize; n],
    };
    let h = n / 2;

    let mut xxx = vec![vec![]; k + 1];
    for bit in 0u32..(1 << h) {
        let idx = bit.count_ones() as usize;
        if k < idx {
            continue;
        }
        let mut sum = 0;
        for i in 0..h {
            if (bit >> i) & 1 != 0 {
                sum += aa[i];
            }
        }
        if sum <= p {
            xxx[idx].push(sum);
        }
    }

    let mut yyy = vec![vec![]; k + 1];
    for bit in 0u32..(1 << (n - h)) {
        let idx = bit.count_ones() as usize;
        if k < idx {
            continue;
        }
        let mut sum = 0;
        for i in 0..(n - h) {
            if (bit >> i) & 1 != 0 {
                sum += aa[h + i];
            }
        }
        if sum <= p {
            yyy[idx].push(sum);
        }
    }
    yyy.iter_mut().for_each(|yy| yy.sort_unstable());
    // for (ky, yy) in yyy.iter().enumerate() {
    //     eprintln!("{ky}: {yy:?}");
    // }

    let mut rs = 0usize;
    for (kx, xx) in xxx.into_iter().enumerate() {
        for x in xx {
            let ky = k - kx;
            let pos = yyy[ky].partition_point(|&y| x + y <= p);
            rs += pos;
        }
    }
    println!("{rs}");
}
