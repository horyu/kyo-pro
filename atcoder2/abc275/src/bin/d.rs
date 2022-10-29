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
        n: usize,
    };
    let rs = f(n, &mut HashMap::new());
    println!("{rs}");
}
fn f(x: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if x == 0 {
        return 1;
    }
    if let Some(&rs) = memo.get(&x) {
        return rs;
    }
    let rs = f(x / 2, memo) + f(x / 3, memo);
    memo.insert(x, rs);
    rs
}
