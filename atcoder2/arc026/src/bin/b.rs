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
    let sum = (1..=n.sqrt().min(n - 1)).fold(0usize, |mut acc, i| {
        let j = n / i;
        if i * j == n {
            acc += i;
            let j = n / i;
            if i != j && j != n {
                acc += j;
            }
        }
        acc
    });
    let rs = match sum.cmp(&n) {
        std::cmp::Ordering::Less => "Deficient",
        std::cmp::Ordering::Equal => "Perfect",
        std::cmp::Ordering::Greater => "Abundant",
    };
    println!("{rs}");
}
