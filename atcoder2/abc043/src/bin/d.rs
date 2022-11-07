#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        s: Chars,
    };
    if s.len() == 2 && s[0] == s[1] {
        println!("1 2");
        return;
    }
    for (i, (c0, c1, c2)) in s.into_iter().tuple_windows().enumerate() {
        if c0 == c1 || c1 == c2 || c2 == c0 {
            println!("{} {}", i + 1, i + 3);
            return;
        }
    }
    println!("-1 -1");
}
