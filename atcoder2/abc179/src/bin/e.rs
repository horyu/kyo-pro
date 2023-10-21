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
        x: usize,
        m: usize,
    };
    let mut a = x;
    let mut ss = vec![0];
    let mut a_to_i = vec![!0usize; m];
    for r in 0..n {
        if a_to_i[a] != !0 {
            let l = a_to_i[a];
            let loop_size = r - l;
            let mut rs = ss[l];
            rs += (ss[r] - ss[l]) * ((n - l) / loop_size);
            rs += ss[l + (n - l) % loop_size] - ss[l];
            println!("{rs}");
            return;
        }
        a_to_i[a] = r;
        ss.push(ss.last().unwrap() + a);
        a = a * a % m;
    }
    let rs = ss.last().unwrap();
    println!("{rs}");
}
