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
        t: usize,
        nnss: [(usize, Chars); t],
    };
    for (n, mut s) in nnss {
        if let Some(mut i) = (0..(n - 1)).find(|&i| s[i + 1] < s[i]) {
            while i + 1 < n && s[i + 1] <= s[i] {
                s.swap(i, i + 1);
                i += 1;
            }
        }
        let rs = s.iter().join("");
        println!("{rs}");
    }
}
