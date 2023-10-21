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
        x: usize,
    };
    //   P
    //  BPPPB
    // BBPPPBPBPPPBB
    let rs = dfs(n, x);
    println!("{rs}");
}

const SIZES: [usize; 51] = {
    let mut arr = [1; 51];
    let mut i = 1;
    while i < 51 {
        arr[i] = 2 * arr[i - 1] + 3;
        i += 1;
    }
    arr
};
const PP: [usize; 51] = {
    let mut arr = [1; 51];
    let mut i = 1;
    while i < 51 {
        arr[i] = 2 * arr[i - 1] + 1;
        i += 1;
    }
    arr
};

fn dfs(n: usize, x: usize) -> usize {
    // eprintln!("{n} {x}");
    if n == 0 {
        assert!(x <= 1, "{x}<=1");
        // x.min(1)
        x
    } else {
        // eprintln!("{}:{} | {:?}", SIZES[n - 1] + 2, x, (SIZES[n - 1] + 2).cmp(&x));
        // dbg!(SIZES[n - 1] + 2, x.cmp(&(SIZES[n - 1] + 2)));
        match x.cmp(&(SIZES[n - 1] + 2)) {
            std::cmp::Ordering::Less => dfs(n - 1, x.saturating_sub(1)),
            std::cmp::Ordering::Equal => PP[n - 1] + 1,
            std::cmp::Ordering::Greater => {
                // dbg!(PP[n - 1] + 1, SIZES[n - 1], );
                PP[n - 1] + 1 + dfs(n - 1, (x - (SIZES[n - 1] + 2)).min(SIZES[n - 1]))
            }
        }
    }
}
