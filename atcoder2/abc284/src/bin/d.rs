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
        t: usize,
        nn: [usize; t],
    };
    for n in nn {
        for i in 2..=n {
            if n % i == 0 {
                let (p, q) = if n / i % i == 0 {
                    (i, n / i / i)
                } else {
                    ((n / i).sqrt(), i)
                };
                println!("{p} {q}");
                break;
            }
        }
    }
}
