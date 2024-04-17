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
        q: usize,
    };
    let mut qq = VecDeque::from_iter((1..=n).map(|i| (i as isize, 0isize)));
    for _ in 0..q {
        input! { t: usize};
        if t == 1 {
            input! { c: char };
            qq.pop_back();
            let (dx, dy) = match c {
                'R' => (1, 0),
                'L' => (-1, 0),
                'U' => (0, 1),
                'D' => (0, -1),
                _ => unreachable!(),
            };
            let (x, y) = qq[0];
            let (nx, ny) = (x + dx, y + dy);
            qq.push_front((nx, ny));
        } else {
            input! { p: Usize1 };
            let (x, y) = qq[p];
            println!("{x} {y}");
        }
    }
}
