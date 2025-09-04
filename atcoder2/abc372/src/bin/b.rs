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
        m: usize,
    };
    let mut mm = m;
    let mut aa = vec![];
    while 0 < mm {
        for i in (0u32..=10).rev() {
            let pow = 3usize.pow(i);
            if pow <= mm {
                aa.push(i);
                mm -= pow;
                break;
            }
        }
    }
    println!("{}", aa.len());
    let rs = aa.iter().join(" ");
    println!("{rs}");
}
