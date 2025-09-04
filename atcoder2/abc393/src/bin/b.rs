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
        s: Chars,
    };
    let len = s.len();
    let mut rs = 0;
    for l in 0..(len - 2) {
        // l + 2d <= len - 1
        // d <= (len - l - 1) / 2
        for d in 1..=((len - l - 1) / 2) {
            if s[l] == 'A' && s[l + d] == 'B' && s[l + 2 * d] == 'C' {
                rs += 1;
            }
        }
    }
    println!("{rs}");
}
