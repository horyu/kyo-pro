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
        s: Chars,
    };
    let mut ll = vec![];
    let mut cc = vec![];
    for c in s {
        match c {
            '(' => {
                cc.push(c);
                ll.push(cc.len());
            }
            ')' => {
                if let Some(l) = ll.pop() {
                    cc.truncate(l.saturating_sub(1));
                } else {
                    cc.push(c);
                }
            }
            _ => {
                cc.push(c);
            }
        }
    }
    let rs = cc.iter().join("");
    println!("{rs}");
}
