#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library::ModInt1000000007;
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    };
    let mut rs = ModInt1000000007::new(1);
    let mut arr = [0usize; 3];
    for a in aa {
        if let Some(l) = arr.iter().copied().position(|x| x == a) {
            let r = arr.iter().copied().rposition(|x| x == a).unwrap();
            rs *= 1 + r - l;
            arr[l] += 1;
        } else {
            println!("0");
            return;
        }
    }
    println!("{rs}");
}
