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
        mut aaa: [[usize; n]; n],
        bbb: [[usize; n]; n],
    };
    for _ in 0..4 {
        // for aa in &aaa {
        //     eprintln!("{}", aa.iter().join(" "));
        // }
        // eprintln!();
        let mut ok = true;
        for i in 0..n {
            for j in 0..n {
                if aaa[i][j] == 1 && bbb[i][j] != 1 {
                    ok = false;
                }
            }
        }
        if ok {
            println!("Yes");
            return;
        }
        let mut ccc = aaa.clone();
        for i in 0..n {
            for j in 0..n {
                ccc[i][j] = aaa[n - 1 - j][i];
            }
        }
        aaa = ccc;
    }
    println!("No");
}
