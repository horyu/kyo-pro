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
        t: Chars,
    };
    let tf = izip!(s, t).all(|(sc, tc)| {
        sc == tc || matches!((sc, tc), ('l', '1') | ('1', 'l') | ('o', '0') | ('0', 'o'))
    });
    let rs = ["No", "Yes"][tf as usize];
    println!("{rs}");
}
