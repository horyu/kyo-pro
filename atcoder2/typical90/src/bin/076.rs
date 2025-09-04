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
        aa: [usize; n],
    };
    let sum = aa.iter().sum::<usize>();
    let target = sum / 10;
    if target * 10 != sum {
        println!("No");
        return;
    }
    let aa = aa.repeat(2);
    // 尺取法
    let mut l = 0;
    let mut crr = 0;
    for r in 0..(2 * n) {
        crr += aa[r];
        while target < crr {
            crr -= aa[l];
            l += 1;
        }
        if crr == target {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
