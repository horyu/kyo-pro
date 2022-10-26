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
        q: usize,
    };
    let mut rs = vec![-1; q];
    for i in 0..q {
        input! {
            s: String,
        };
        match s.as_str() {
            "ADD" => {
                input! {x: isize};
            }
            "DELETE" => {}
            "SAVE" => {
                input! {y: isize};
            }
            _ => {
                input! {z: isize};
            }
        }
    }
    println!("{}", rs.into_iter().join(" "));
}
