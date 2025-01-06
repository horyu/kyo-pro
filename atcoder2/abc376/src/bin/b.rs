#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
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
        q: usize,
        hhtt: [(char, Usize1); q],
    };
    let mut rs = 0;
    let mut l = 0;
    let mut r = l + 1;
    for (h, t) in hhtt {
        let (x, y) = if h == 'L' { (&mut l, r) } else { (&mut r, l) };
        if *x == t {
            continue;
        }
        // txy
        if t < *x && *x < y {
            rs += *x - t;
        }
        // tyx
        else if t < y && y < *x {
            rs += n - *x + t;
        }
        // xty
        else if *x < t && t < y {
            rs += t - *x;
        }
        // xyt
        else if *x < y && y < t {
            rs += n - t + *x;
        }
        // yxt
        else if y < *x && *x < t {
            rs += t - *x;
        }
        // ytx
        else if y < t && t < *x {
            rs += *x - t;
        } else {
            unreachable!();
        }
        *x = t;
    }
    println!("{rs}");
}
