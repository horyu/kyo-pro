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
        fff: [[usize; 10]; n],
        ppp: [[isize; 11]; n],
    };
    let mut rs = isize::MIN;
    for bit in 1usize..(1 << 10) {
        let mut tmp = 0;
        for i in 0..n {
            let mut cnt = 0;
            for j in 0..10 {
                if bit & (1 << j) != 0 && fff[i][j] == 1 {
                    cnt += 1;
                }
            }
            tmp += ppp[i][cnt];
        }
        rs = rs.max(tmp);
    }
    println!("{rs}");
}
