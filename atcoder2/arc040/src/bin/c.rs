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
        ss: [Chars; n],
    };
    let mut rs = 0;
    let mut rr = n;
    for s in ss {
        if let Some(r) = s[..rr].iter().copied().rposition(|c| c == '.') {
            rs += 1;
            rr = r;
        } else {
            rr = n;
        }
    }
    println!("{rs}");
}
