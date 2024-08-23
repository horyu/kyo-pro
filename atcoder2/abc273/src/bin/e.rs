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
    let mut vv = vec![];
    // ppxx[i] = (クエリ処理前のppxxインデックス, クエリ処理後の整数列の末尾)
    let mut ppxx = vec![];
    let mut cur = None;
    let mut hm = HashMap::new();
    for _ in 0..q {
        input! { s: String };
        match s.as_str() {
            "ADD" => {
                input! {x: isize};
                ppxx.push((cur, x));
                cur = Some(ppxx.len() - 1);
            }
            "DELETE" => {
                if let Some(i) = cur {
                    cur = ppxx[i].0;
                }
            }
            "SAVE" => {
                input! {y: usize};
                hm.insert(y, cur);
            }
            // LOAD
            _ => {
                input! {z: usize};
                cur = hm.get(&z).copied().unwrap_or_default();
            }
        }
        if let Some(i) = cur {
            vv.push(ppxx[i].1);
        } else {
            vv.push(-1)
        }
    }
    let rs = vv.into_iter().join(" ");
    println!("{rs}");
}
