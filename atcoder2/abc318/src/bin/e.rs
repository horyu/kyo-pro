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
        aa: [usize; n],
    };
    let mut mm = multimap::MultiMap::new();
    for (i, a) in aa.iter().copied().enumerate() {
        mm.insert(a, i);
    }
    let mut rs = 0usize;
    for (_, ii) in mm {
        let mut left = 0;
        let mut right = ii.len();
        for (i, j) in ii.iter().copied().tuple_windows() {
            left += 1;
            right -= 1;
            rs += (j - i - 1) * left * right;
            // eprintln!("{i} {j} {}", (j - i) * left * right);
        }
    }
    println!("{rs}");
    // let mut rs = 0;
    // for i in 0..n {
    //     for j in (i + 1)..n {
    //         for k in (j + 1)..n {
    //             if aa[i] == aa[k] && aa[i] != aa[j] {
    //                 eprintln!("{i} {j} {k}:{} {} {}", aa[i], aa[j], aa[k]);
    //                 rs += 1;
    //             }
    //         }
    //     }
    // }
    // dbg!(rs);
}
