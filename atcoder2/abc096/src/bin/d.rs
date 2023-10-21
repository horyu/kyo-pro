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
    };
    const MAX: usize = 55555;
    let mut arr = [true; MAX];
    arr[0] = false;
    arr[1] = false;
    for i in 2..MAX {
        if arr[i] {
            let mut j = i * 2;
            while j < MAX {
                arr[j] = false;
                j += i;
            }
        }
    }
    // 5k-1
    let xx = (1..MAX)
        .filter(|&i| arr[i] && (i + 1) % 5 == 0)
        .take(n)
        .collect_vec();
    // dbg!(xx.len());
    println!("{}", xx.iter().join(" "));
}
