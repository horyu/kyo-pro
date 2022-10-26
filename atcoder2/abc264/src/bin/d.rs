#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        s: Chars
    };
    let mut ii = s
        .into_iter()
        .map(|c| match c {
            'a' => 0,
            't' => 1,
            'c' => 2,
            'o' => 3,
            'd' => 4,
            'e' => 5,
            'r' => 6,
            _ => unreachable!(),
        })
        .collect_vec();
    let mut rs = 0;
    while ii.iter().tuple_windows().any(|(l, r)| l > r) {
        for i in 0..6 {
            if ii[i] > ii[i + 1] {
                ii.swap(i, i + 1);
                rs += 1;
                break;
            }
        }
    }
    println!("{rs}");
}
