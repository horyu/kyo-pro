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
        n: isize,
        a: isize,
        b: isize,
        c: isize,
    };
    let mut rs = 10000;
    for i in (0..=(n / a).min(9999)).rev() {
        for j in (0..=((n - a * i) / b).min(9999)).rev() {
            let k = (n - a * i - b * j) / c;
            if a * i + b * j + c * k == n {
                rs = rs.min(i + j + k);
            }
        }
    }
    println!("{rs}");
}
