#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
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
        mut aa: [usize; n],
    };
    // https://atcoder.jp/contests/arc160/editorial/6338
    let mut x = 1;
    let mut y = n * (n + 1) / 2;
    for l in 0..n {
        let mut sml = vec![];
        let mut big = vec![];
        for r in (l + 1)..n {
            if aa[r] < aa[l] {
                sml.push(aa[r]);
            } else {
                big.push(aa[r]);
            }
        }
        let mut v = !0;
        if k - x < sml.len() {
            sml.sort_unstable();
            v = sml[k - x];
        }
        if y - k < big.len() {
            big.sort_unstable_by_key(|&b| R(b));
            v = big[y - k];
        }
        if v != !0 {
            let j = l + aa[l..].iter().position(|&a| a == v).unwrap();
            aa[l..=j].reverse();
            break;
        }
        x += sml.len();
        y -= big.len();
    }
    let rs = aa.into_iter().join(" ");
    println!("{rs}");
}
