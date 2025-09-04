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
        x: usize,
        mut aa: [usize; n],
    };
    aa.push(0);
    let mut rs = 0usize;
    for i in 0..n {
        let sum = aa[i] + aa[i + 1];
        if x < sum {
            let diff = sum - x;
            rs += diff;
            if let Some(ar) = aa[i + 1].checked_sub(diff) {
                aa[i + 1] = ar;
            } else {
                aa[i + 1] = 0;
                // aa[i] 更新
            }
        }
    }
    println!("{rs}");
}
