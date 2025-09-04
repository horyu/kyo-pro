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
        ss: [Chars; 8],
    };
    let mut ii = [true; 8];
    let mut jj = [true; 8];
    for (i, s) in ss.iter().enumerate() {
        for (j, c) in s.iter().copied().enumerate() {
            if c == '#' {
                ii[i] = false;
                jj[j] = false;
            }
        }
    }
    let mut rs = 0;
    for i in 0..8 {
        if ii[i] {
            for j in 0..8 {
                if jj[j] {
                    rs += 1;
                }
            }
        }
    }
    println!("{rs}");
}
