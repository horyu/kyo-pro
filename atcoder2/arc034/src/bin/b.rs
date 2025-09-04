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
    };
    // f(a) = aの各桁の和
    // n = a + f(a), n <= 1e18

    // ある数xについてf(x)の最大値は 9*(xの桁数)
    let mut rrss = vec![];
    let ilog10 = n.ilog10();
    for d in 1.max(ilog10)..=(18.min(ilog10 + 1)) {
        // eprintln!("d={d}");
        for f in 1..=(9 * d as usize) {
            if let Some(a) = n.checked_sub(f) {
                let mut x = a;
                let mut sum = 0;
                while x > 0 {
                    sum += x % 10;
                    x /= 10;
                }
                if sum == f {
                    rrss.push(a);
                }
            }
        }
    }
    rrss.sort_unstable();
    rrss.dedup();
    println!("{}", rrss.len());
    for rs in rrss {
        println!("{}", rs);
    }
}
