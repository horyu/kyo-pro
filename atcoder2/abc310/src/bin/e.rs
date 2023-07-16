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
        s: Chars,
    };
    let mut rs = 0usize;
    let mut cc = [0; 2];
    for c in s {
        if c == '0' {
            cc = [1, cc[0] + cc[1]];
        } else {
            cc = [cc[1], cc[0] + 1];
        }
        rs += cc[1];
    }
    println!("{rs}");
}
