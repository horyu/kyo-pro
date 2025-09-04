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
        n: usize,
        s: Chars,
    };
    let mut rs = 1;
    for pos in s.iter().copied().positions(|c| c == '/') {
        let mut next = 1;
        while let (Some('1'), Some('2')) = (s.get(pos.overflowing_sub(next).0), s.get(pos + next)) {
            next += 1;
        }
        rs = rs.max(1 + (next - 1) * 2);
    }
    println!("{rs}");
}
