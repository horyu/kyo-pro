#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        t: usize,
    };
    for _ in 0..t {
        input! {
            n: usize,
            mut ss: [usize; n],
        };
        let last = ss.pop().unwrap();
        let mut x = ss.swap_remove(0);
        let bts = BTreeSet::from_iter(ss);
        let mut rs = 2;
        while 2 * x < last {
            if let Some(&y) = bts.range((x + 1)..=(2 * x)).next_back() {
                rs += 1;
                x = y;
            } else {
                break;
            }
        }
        if 2 * x < last {
            println!("-1");
        } else {
            println!("{rs}");
        }
    }
}
