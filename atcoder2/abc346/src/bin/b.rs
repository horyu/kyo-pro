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
        w: usize,
        b: usize,
    };
    const BASE: &str = "wbwbwwbwbwbw";
    let s = BASE.repeat(100).chars().collect::<Vec<char>>();
    for i in 0..BASE.len() {
        let mut ww = 0;
        let mut bb = 0;
        let mut j = i;
        while ww <= w && bb <= b {
            if s[j] == 'w' {
                ww += 1;
            } else {
                bb += 1;
            }
            if ww == w && bb == b {
                println!("Yes");
                return;
            }
            j += 1;
        }
    }
    println!("No");
}
