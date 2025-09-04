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
        mut s: Chars,
    };
    let mut kk = 0;
    for i in 0..n {
        if s[i] == 'o' {
            kk += 1;
            if let Some(c) = s.get_mut(i.wrapping_sub(1)) {
                *c = '.';
            }
            if let Some(c) = s.get_mut(i + 1) {
                *c = '.';
            }
        }
    }
    let iii = (0..n)
        .group_by(|i| s[*i] == '?')
        .into_iter()
        .filter_map(|(tf, g)| tf.then(|| g.collect_vec()))
        .collect_vec();
    let max = iii.iter().fold(0, |acc, ii| acc + ii.len().div_ceil(2));
    if k == kk {
        for ii in iii {
            for i in ii {
                s[i] = '.';
            }
        }
    } else if max == k - kk {
        for ii in iii {
            if ii.len() % 2 == 0 {
                continue;
            }
            let i0 = ii[0];
            for i in ii {
                s[i] = ['o', '.'][(i - i0) % 2];
            }
        }
    };
    let rs = s.iter().join("");
    println!("{rs}");
}
