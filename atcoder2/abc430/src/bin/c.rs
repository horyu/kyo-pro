#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[cfg(not(debug_assertions))]
macro_rules! eprintln {
    ($($tt:tt)*) => {};
}

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        s: Chars,
    };
    let mut cnts = vec![[0; 2]; n];
    for (i, c) in s.iter().copied().enumerate() {
        if 0 < i {
            cnts[i] = cnts[i - 1];
        }
        cnts[i][(c == 'b') as usize] += 1;
    }
    let mut rs = 0usize;
    for l in 0..(n.saturating_sub(a)) {
        let [la, lb] = cnts[l];
        // 'a' がa個以上になる左端インデックス
        let j_a = l + cnts[l..].partition_point(|cc| cc[0] - la < a) + 1;
        // 'b'がb個未満の右端インデックス
        let j_b = l + cnts[l..].partition_point(|cc| cc[1] - lb <= b) - 1;
        eprintln!("[{l}, {j_a}, {j_b}]");
    }
    println!("{rs}");
}
