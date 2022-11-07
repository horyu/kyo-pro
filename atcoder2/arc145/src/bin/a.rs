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
        n: usize,
        mut s: Chars,
    };
    if n == 2 {
        println!("{}", ["No", "Yes"][(s[0] == s[n - 1]) as usize]);
    } else {
        println!(
            "{}",
            ["Yes", "No"][(s[0] == 'A' && s[n - 1] == 'B') as usize]
        );
    }
}
