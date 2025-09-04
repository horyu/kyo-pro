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
        a: usize,
        b: usize,
    };
    // F(A, B) = F(0, B) ^ F(0, A - 1)
    // G(X) = F(0, X) = (0 ^ 1) ^ (2 ^ 3) ^ ... ^ X = 1 ^ 1 ^ ... ^ X
    let g = |x: usize| -> usize {
        let mut tmp = x.div_ceil(2).is_odd() as usize;
        if x.is_even() {
            tmp ^= x;
        }
        tmp
    };
    let rs = if a == 0 { g(b) } else { g(a - 1) ^ g(b) };
    println!("{rs}");
}
