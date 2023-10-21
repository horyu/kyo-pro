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
        s: Chars,
    };
    let mut rs = 0;
    for i in 0..s.len() {
        for j in (i + 1)..=s.len() {
            if izip!(i..j, (i..j).rev()).all(|(a, b)| s[a] == s[b]) {
                rs = rs.max(j - i);
            }
        }
    }
    println!("{rs}");
}
