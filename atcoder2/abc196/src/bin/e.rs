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
        aatt: [(isize, usize); n],
        q: usize,
        xx: [isize; q],
    };
    // https://atcoder.jp/contests/abc196/editorial/948
    let mut low = isize::MIN / 2;
    let mut high = isize::MAX / 2;
    let mut add = 0;
    for (a, t) in aatt {
        match t {
            1 => {
                low += a;
                high += a;
                add += a;
            }
            2 => {
                low = low.max(a);
                high = high.max(a);
            }
            3 => {
                low = low.min(a);
                high = high.min(a);
            }
            _ => unreachable!(),
        }
    }
    for x in xx {
        let rs = high.min(low.max(x + add));
        println!("{rs}");
    }
}
