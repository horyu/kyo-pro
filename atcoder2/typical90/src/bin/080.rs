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
        d: usize,
        aa: [usize; n],
    };
    let mut rs = 0isize;
    for i in 0..(1 << n) {
        let mut bit = 0;
        let mut conditions = 0;
        for j in 0..n {
            if (i >> j) & 1 == 1 {
                bit |= aa[j];
                conditions += 1;
            }
        }
        let mut free_digits = 0;
        for j in 0..d {
            if (bit >> j) & 1 == 0 {
                free_digits += 1;
            }
        }
        if conditions.is_even() {
            rs += 1 << free_digits;
        } else {
            rs -= 1 << free_digits;
        }
    }
    println!("{rs}");
}
