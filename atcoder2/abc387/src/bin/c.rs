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
        l: usize,
        r: usize,
    };
    let rs = f(r) - f(l - 1);
    println!("{rs}");
}

fn f(n: usize) -> usize {
    if n < 10 {
        return n;
    }
    // 325 -> c(1**) + c(2**) + c(30*) + c(31*) + c(32*)
    // 231 -> c(1**) + c(20*) + c(21*)

    let log = n.ilog10();
    let pow = 10usize.pow(log);
    let head = n / pow;
    // head.saturating_sub(1) * pow + f(n % pow)
    (1..head).fold(0, |acc, i| i.pow(log)) + g(n % pow, head)
}

fn g(n: usize, max: usize) -> usize {
    n
}
