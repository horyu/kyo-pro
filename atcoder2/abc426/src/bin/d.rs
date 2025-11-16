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
    for (n, s) in nnss {
        let mut ww = vec![[0; 2]; n + 1];
        for (i, c) in s.iter().copied().enumerate() {
            ww[i + 1] = ww[i];
            ww[i + 1][usize::from(c == '1')] += 1;
        }
        let mut rs = !0usize;
        let mut l = 0;
        for pos in [0, 1] {
            eprintln!("{}", (0..=n).map(|i| ww[i][pos]).join(""));
        }
        while l < n {
            let cl = s[l];
            let pos = usize::from(cl == '1');
            let r = l + s[l..].iter().take_while(|&&c| c == cl).count() - 1;
            let left = ww[l][pos] * 2 + ww[l][1 ^ pos];
            let right = (ww[n][pos] - ww[r + 1][pos]) * 2 + (ww[n][1 ^ pos] - ww[r + 1][1 ^ pos]);
            let cnt = left + right;
            rs = rs.min(cnt);
            eprintln!("[{n}] {l}-{r}: {cnt}({left} + {right})");
            l = r + 1;
        }
        println!("{rs}");
    }
}
