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
        ss: [Chars; 8],
    };
    for i in 0..8 {
        for j in 0..8 {
            if ss[i][j] == '*' {
                let y = (b'0' + (8 - i) as u8) as char;
                let x = (b'a' + j as u8) as char;
                println!("{x}{y}");
                return;
            }
        }
    }
}
