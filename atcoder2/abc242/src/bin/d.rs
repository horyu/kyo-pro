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
        s: Bytes,
        q: usize,
        ttkk: [(usize, Usize1); q],
    };
    for (mut t, mut k) in ttkk {
        let mut diff = 0u8;
        while 0 < t {
            diff = (diff + [1, 2][k % 2]) % 3;
            t -= 1;
            k /= 2;
            if k == 0 {
                t %= 3;
            }
        }
        let rs = (b'A' + (s[k] - b'A' + diff) % 3) as char;
        println!("{rs}");
    }
}
