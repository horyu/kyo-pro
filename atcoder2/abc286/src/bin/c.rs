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
        a: usize,
        b: usize,
        mut s: Chars,
    };
    let mut rs = !0;
    for rotate in 0..n {
        let mut cnt = rotate * a;
        for i in 0..(n / 2) {
            if s[i] != s[n - 1 - i] {
                cnt += b;
            }
        }
        rs = rs.min(cnt);
        s.rotate_left(1);
    }
    println!("{rs}");
}
