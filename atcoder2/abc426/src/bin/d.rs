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

#[cfg(not(debug_assertions))]
macro_rules! eprintln {
    ($($tt:tt)*) => {};
}

fn main() {
    input! {
        t: usize,
        nnss: [(usize, Chars); t]
    };
    for (n , s) in nnss {
        let mut ww  = vec![[0; 2]; n + 1];
        for (i, c) in s.iter().copied().enumerate() {
            ww[i + 1] = ww[i];
            ww[i + 1][usize::from(c == '1')] += 1;
        }
        let mut rs = !0;
        let mut l = 0;
        while l < n {
            let cl = s[l];
            let r = l + s[l..].iter().position(|&c| c == cl).unwrap();
            let mut cnt = 0;
            // TODO
            l = r + 1;
            rs = rs.min(cnt);
        }

        println!("{rs}");
    }
}
