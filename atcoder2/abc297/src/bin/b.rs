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
    let tf = s.iter().position(|&c| c == 'B').unwrap() % 2
        != s.iter().rposition(|&c| c == 'B').unwrap() % 2
        && (s.iter().position(|&c| c == 'R').unwrap()
            ..(s.iter().rposition(|&c| c == 'R').unwrap()))
            .any(|i| s[i] == 'K');
    let rs = ["No", "Yes"][tf as usize];
    println!("{rs}");
}
