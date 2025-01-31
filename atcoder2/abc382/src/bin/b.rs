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
        mut d: usize,
        mut s: Chars,
    };
    for c in s.iter_mut().rev() {
        if *c == '@' {
            *c = '.';
            d -= 1;
            if d == 0 {
                break;
            }
        }
    }
    let rs = s.iter().join("");
    println!("{rs}");
}
