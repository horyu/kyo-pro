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
        ss: [String; n],
    };
    let mut login = false;
    let mut rs = 0;
    for s in ss {
        match s.as_str() {
            "login" => login = true,
            "logout" => login = false,
            "public" => (),
            "private" => rs += i32::from(!login),
            _ => unreachable!(),
        }
    }
    println!("{rs}");
}
