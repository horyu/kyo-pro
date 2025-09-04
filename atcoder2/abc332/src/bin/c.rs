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
        m: usize,
        s: Chars,
    };
    let mut c1 = 0usize;
    let mut c2 = 0;
    let mut rs = 0;
    for c in chain!(s, ['0']) {
        match c {
            '0' => {
                rs = rs.max(c1.saturating_sub(m) + c2);
                c1 = 0;
                c2 = 0;
            }
            '1' => {
                c1 += 1;
            }
            '2' => {
                c2 += 1;
            }
            _ => unreachable!(),
        }
    }
    println!("{rs}");
}
