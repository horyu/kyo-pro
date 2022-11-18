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
        aa: [usize; n],
        bb: [usize; n],
        cc: [usize; n],
    };
    const SIZE: usize = 46;
    let to_arr = |vv: Vec<usize>| -> [usize; SIZE] {
        let mut arr = [0; SIZE];
        for v in vv {
            arr[v % SIZE] += 1;
        }
        arr
    };
    let aa = to_arr(aa);
    let bb = to_arr(bb);
    let cc = to_arr(cc);
    let mut rs = 0usize;
    for i in 0..SIZE {
        for j in 0..SIZE {
            let k = (SIZE - (i + j) % SIZE) % SIZE;
            rs += aa[i] * bb[j] * cc[k];
        }
    }
    println!("{rs}");
}
