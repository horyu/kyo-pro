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
        aa: [isize; n],
    };
    // arr[i] = モンスターを i%2 回倒したときの経験値合計の最大値
    let mut arr = [0, isize::MIN];
    for a in aa {
        arr = [arr[0].max(arr[1] + 2 * a), arr[1].max(arr[0] + a)];
    }
    let rs = arr.iter().max().unwrap();
    println!("{rs}");
}
