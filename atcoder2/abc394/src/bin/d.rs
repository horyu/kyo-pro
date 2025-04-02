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
        s: Chars,
    };
    let mut vv = vec![];
    for c in s {
        match c {
            ')' => {
                if vv.pop_if(|x| *x == '(').is_some() {
                    continue;
                }
            }
            ']' => {
                if vv.pop_if(|x| *x == '[').is_some() {
                    continue;
                }
            }
            '>' => {
                if vv.pop_if(|x| *x == '<').is_some() {
                    continue;
                }
            }
            _ => {}
        }
        vv.push(c);
    }
    let rs = ["No", "Yes"][vv.is_empty() as usize];
    println!("{rs}");
}
