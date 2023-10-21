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
        n: usize,
        s: Chars,
    };
    let mut cc = s.clone();
    let mut is_inner = true;
    for (i, c) in s.into_iter().enumerate() {
        match c {
            '"' => {
                is_inner = !is_inner;
            }
            ',' => {
                if is_inner {
                    cc[i] = '.';
                }
            }
            _ => {}
        }
    }
    let rs = cc.iter().join("");
    println!("{rs}");
}
