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
        s: String
    };
    let tf = "が ACE、BDF、CEG、DFA、EGB、FAC、GBD のいずれかと等しいとき".contains(s.as_str());
    // let tf = matches!(
    //     s.as_ref(),
    //     "ACE" | "BDF" | "CEG" | "DFA" | "EGB" | "FAC" | "GBD"
    // );
    let rs = ["No", "Yes"][tf as usize];
    println!("{rs}");
}
