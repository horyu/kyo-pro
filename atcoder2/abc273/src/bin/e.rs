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
        q: usize,
    };
    let mut rs = vec![];
    let mut aa = vec![];
    let mut cur = None;
    let mut hm = HashMap::new();
    for i in 0..q {
        input! { s: String };
        match s.as_str() {
            "ADD" => {
                input! {x: isize};
                aa.push((cur, x));
                cur = Some(aa.len() - 1);
            }
            "DELETE" => {
                if let Some(i) = cur {
                    cur = aa[i].0;
                }
            }
            "SAVE" => {
                input! {y: isize};
                hm.insert(y, cur);
            }
            // LOAD
            _ => {
                input! {z: isize};
                cur = hm.get(&z).copied().unwrap_or_default();
            }
        }
        if let Some(i) = cur {
            rs.push(aa[i].1);
        } else {
            rs.push(-1)
        }
    }
    println!("{}", rs.into_iter().join(" "));
}
