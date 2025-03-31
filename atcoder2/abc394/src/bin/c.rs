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
        mut s: Chars,
    };
    let n = s.len();
    let mut l = 1;
    while l < n {
        if ('W', 'A') == (s[l - 1], s[l]) {
            s[l - 1] = 'A';
            s[l] = 'C';
            l = 1.max(l - 1);
            continue;
        }
        l += 1;
    }
    let rs = s.iter().collect::<String>();
    println!("{rs}");
}
