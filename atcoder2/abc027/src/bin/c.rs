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
    };
    // https://qiita.com/hamamu/items/55433210be3c47a4dd72
    let mut y = n + 1;
    loop {
        let z = y.div_ceil(2);
        if z == 1 {
            println!("Aoki");
            return;
        }
        y = (z - 1).div_ceil(2);
        if y == 1 {
            println!("Takahashi");
            return;
        }
    }
}
