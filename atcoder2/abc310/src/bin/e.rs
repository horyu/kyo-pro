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
        s: Chars,
    };
    // https://atcoder.jp/contests/abc310/editorial/6784
    let mut rs = 0usize;
    let mut zero = 0;
    let mut one = 0;
    for c in s {
        if c == '0' {
            (zero, one) = (1, zero + one);
        } else {
            (zero, one) = (one, zero + 1);
        }
        rs += one;
    }
    println!("{rs}");
}
