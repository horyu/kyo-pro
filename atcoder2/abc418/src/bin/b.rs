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
        s: Chars,
    };
    let n = s.len();
    let mut rs = 0.0f64;
    for i in 0..=n.saturating_sub(3) {
        if s[i] != 't' {
            continue;
        }
        let mut x = 0;
        for j in i..n {
            if s[j] != 't' {
                continue;
            }
            x += 1;
            let len = j - i + 1;
            if 3 <= len {
                rs = rs.max((x - 2) as f64 / (len - 2) as f64);
            }
        }
    }
    println!("{rs}");
}
