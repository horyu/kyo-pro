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

/*
可能な限り分散するようにkkを分配する
2の冪乗ごとに分配するイメージ
12345678
1 3 2 4
 5 7 6 8
01234567
1       :0+[0]
    2   :4+[0] -> [0,4]
  3   4 :2+[0,4] -> [0,4,2,6]
 5 7 6 8:1+[0,4,2,6]

1234567890123456
1   3   2   4
  5   7   6   8
 9   1   0   2
   3   5   4   6
0123456789012345
*/

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    let size = 2usize.pow(n as u32);
    let mut bb = vec![k / size; size];
    // TODO
    let rs = bb.iter().join(" ");
    println!("{rs}");
}
