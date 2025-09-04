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
        mut a: usize,
        mut b: usize,
    };
    let mut rs = 0;
    while a != b {
        if b < a {
            std::mem::swap(&mut a, &mut b);
        }
        // 引けるだけ引く
        let diff = b - a;
        if 0 < diff {
            let cnt = diff.div_ceil(a);
            rs += cnt;
            b -= cnt * a;
        }
    }
    println!("{rs}");
}
