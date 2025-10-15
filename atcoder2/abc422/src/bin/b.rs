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
        h: usize,
        w: usize,
        ss: [Chars; h],
    };
    for (i, s) in ss.iter().enumerate() {
        for (j, c) in s.iter().copied().enumerate() {
            if c == '#' {
                let mut cnt = 0;
                for (dx, dy) in [(0, 1), (0, !0), (1, 0), (!0, 0)] {
                    let ni = i.wrapping_add(dy);
                    let nj = j.wrapping_add(dx);
                    if let Some(s) = ss.get(ni)
                        && s.get(nj) == Some(&'#')
                    {
                        cnt += 1;
                    }
                }
                if !matches!(cnt, 2 | 4) {
                    println!("No");
                    return;
                }
            }
        }
    }
    println!("Yes");
}
