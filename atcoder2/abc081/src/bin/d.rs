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
        mut aa: [isize; n],
    };
    let mut rs = vec![];
    while let itertools::MinMaxResult::MinMax(min_pos, max_pos) =
        aa.iter().copied().position_minmax()
    {
        if min_pos == 0 && max_pos == n - 1 {
            break;
        }
        let min = aa[min_pos];
        let max = aa[max_pos];
        if min.abs() <= max.abs() {
            if let Some(i) = (0..(n - 1)).position(|i| aa[i + 1] < aa[i]) {
                aa[i + 1] += max;
                rs.push([max_pos + 1, i + 2]);
            }
        } else {
            if let Some(i) = (0..(n - 1)).rposition(|i| aa[i + 1] < aa[i]) {
                aa[i] += min;
                rs.push([min_pos + 1, i + 1]);
            }
        }
        // eprintln!("{}", aa.iter().join(" "));
    }
    println!("{}", rs.len());
    for rs in rs {
        println!("{}", rs.iter().join(" "));
    }
}
