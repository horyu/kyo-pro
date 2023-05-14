#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        s: Chars
    };
    let mut a = 0;
    let mut b = 0;
    let mut rs = 'a';
    for c in s {
        if c == 'T' {
            a += 1;
            if b < a {
                rs = c;
            }
        } else {
            b += 1;
            if a < b {
                rs = c;
            }
        }
    }
    println!("{rs}");
}
