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
    let n = s.len();
    let mut i = 0;
    for c in s {
        if i % 2 == 0 {
            if c == 'o' {
                i += 1;
            }
        } else {
            if c == 'i' {
                i += 1;
            }
        }
        i += 1;
    }
    i = i.div_ceil(&2) * 2;
    let rs = i - n;
    println!("{rs}");
}
