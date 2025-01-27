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
    if n.is_odd() {
        let (left, right) = s.split_at((n - 1) / 2);
        if left.iter().all(|&c| c == '1')
            && right[0] == '/'
            && right.iter().skip(1).all(|&c| c == '2')
        {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
