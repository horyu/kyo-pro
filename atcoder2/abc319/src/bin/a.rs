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
        s: String,
    };
    let rs = match s.as_str() {
        "tourist" => 3858,
        "ksun48" => 3679,
        "Benq" => 3658,
        "Um_nik" => 3648,
        "apiad" => 3638,
        "Stonefeang" => 3630,
        "ecnerwala" => 3613,
        "mnbvmar" => 3555,
        "newbiedmy" => 3516,
        "semiexp" => 3481,
        _ => unreachable!(),
    };
    println!("{rs}");
}
