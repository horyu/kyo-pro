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
        s: Chars,
        n: usize,
    };
    let min = s
        .iter()
        .copied()
        .fold(0, |acc, c| (acc << 1) | (c == '1') as usize);
    if n < min {
        println!("-1");
        return;
    }
    let len = s.len();
    let mut rs = min;
    for (i, c) in s.iter().copied().enumerate() {
        if c == '?' {
            let v = rs | (1 << (len - 1 - i));
            if v <= n {
                rs = v;
            }
        }
    }
    println!("{rs}");
}
