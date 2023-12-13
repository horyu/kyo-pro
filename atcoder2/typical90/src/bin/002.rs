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
    // https://drken1215.hatenablog.com/entry/2021/06/12/151200
    for cc in (0..n).map(|_| ['(', ')']).multi_cartesian_product() {
        let mut score = 0;
        for c in cc.iter().copied() {
            if c == '(' {
                score += 1;
            } else {
                score -= 1;
            }
            if score < 0 {
                break;
            }
        }
        if score == 0 {
            println!("{}", cc.iter().join(""));
        }
    }
}
