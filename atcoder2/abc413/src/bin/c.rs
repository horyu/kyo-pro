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

fn main() {
    input! {
        q: usize,
    };
    let mut qq = VecDeque::new();
    for _ in 0..q {
        input! { t: usize };
        if t == 1 {
            input! { c: usize, x: usize };
            qq.push_back((c, x));
        } else {
            input! { mut k: usize };
            let mut rs = 0;
            while 0 < k
                && let Some((c, x)) = qq.pop_front()
            {
                if c <= k {
                    rs += x * c;
                } else {
                    rs += x * k;
                    qq.push_front((c - k, x));
                }
                k = k.saturating_sub(c);
            }
            println!("{rs}");
        }
    }
}
