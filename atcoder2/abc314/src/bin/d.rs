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
        mut s: Chars,
        q: usize,
        ttxxcc: [(usize, usize, char); q],
    };
    if let Some(last) = ttxxcc.iter().rposition(|txc| txc.0 != 1) {
        for (t, x, c) in ttxxcc[..last].iter().copied() {
            if t == 1 {
                s[x - 1] = c;
            }
        }
        if ttxxcc[last].0 == 2 {
            for c in s.iter_mut() {
                *c = c.to_ascii_lowercase();
            }
        } else {
            for c in s.iter_mut() {
                *c = c.to_ascii_uppercase();
            }
        }
        for (t, x, c) in ttxxcc[(last + 1)..].iter().copied() {
            s[x - 1] = c;
        }
    } else {
        for (t, x, c) in ttxxcc.iter().copied() {
            s[x - 1] = c;
        }
    }
    let rs = s.iter().join("");
    println!("{rs}");
}
