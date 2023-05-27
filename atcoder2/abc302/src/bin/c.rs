#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        ss: [Chars; n],
    };
    for ss in ss.into_iter().permutations(n) {
        if ss
            .iter()
            .tuple_windows()
            .all(|(x, y)| izip!(x, y).fold(0, |acc, (xc, yc)| acc + (xc != yc) as usize) == 1)
        {
            println!("Yes");
            return;
        }
        // for s in ss {
        //     print!("{} ", s.iter().join(""));
        // }
        // println!();
    }
    println!("No");
}