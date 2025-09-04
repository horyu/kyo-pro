#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use ac_library::ModInt998244353;
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

/*
a % b = c, c < b < a <= n
(b,c) a
(2,1) 3,5,7 @ 9
(3,1) 4,7 @ 10
(3,2) 5 @ 8,11
(4,1) 5 @ 9,13
(4,2) 6 @ 10,14
(4,3) 7 @ 11,15
(5,1) 6 @ 11,14
(5,2) 7 @ 12,17
(5,3) @ 8,13,18
(5,4) @ 9,14,19
(6,1) 7 @ 13,19
(6,2) @ 8,14,20
(6,3) @ 9,15,21
(6,4) @ 10,16,22
(6,5) @ 11,17,23
*/

fn main() {
    input! {
        n: usize,
    };
    // https://atcoder.jp/contests/abc414/editorial/13455
    let mut rs = ModInt998244353::new(n - 1) * (n - 2) / 2;
    for b in 2..=n.sqrt() {
        rs -= (n / b - b) * 2 + 1;
    }
    println!("{rs}");
}
