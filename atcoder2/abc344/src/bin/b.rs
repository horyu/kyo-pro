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
use std::io::BufRead;

fn main() {
    use std::io;
    let mut input = io::BufReader::new(io::stdin());
    let mut vv = vec![];
    let mut buf = String::new();
    while input.read_line(&mut buf).is_ok() {
        if &buf == "0\n" {
            break;
        }
        vv.push(std::mem::take(&mut buf));
    }
    let mut rs = String::new();
    for v in vv.into_iter().rev() {
        rs.push_str(&v);
    }
    print!("0\n{rs}");
}
