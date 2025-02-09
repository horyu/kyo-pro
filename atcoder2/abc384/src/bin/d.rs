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
        s: usize,
        aa: [usize; n],
    };
    let a_sum = aa.iter().sum::<usize>();
    let s = s % a_sum;
    let aa = aa.repeat(2);
    // 尺取法
    let mut l = 0;
    let mut sum = 0;
    for r in 0..aa.len() {
        sum += aa[r];
        while s < sum {
            sum -= aa[l];
            l += 1;
        }
        if sum == s {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
