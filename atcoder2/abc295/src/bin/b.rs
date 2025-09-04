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
        r: usize,
        c: usize,
        bbb: [Chars; r],
    };
    let mut rs = bbb.clone();
    for (i, bb) in bbb.into_iter().enumerate() {
        for (j, b) in bb.into_iter().enumerate() {
            if let Some(n) = b.to_digit(10) {
                let n = n as usize;
                for y in 0..r {
                    for x in 0..c {
                        if y.abs_diff(i) + x.abs_diff(j) <= n {
                            rs[y][x] = '.';
                        }
                    }
                }
            }
        }
    }
    for rs in rs {
        println!("{}", rs.iter().join(""));
    }
}
