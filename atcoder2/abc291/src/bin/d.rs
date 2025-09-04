#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use ac_library::ModInt998244353;
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
        aabb: [[usize; 2]; n],
    };
    let mut arr = [ModInt998244353::new(1); 2];
    let mut pre = [!0; 2];
    for ab in aabb {
        let ab = [ab[0], ab[1]];
        let mut arr2 = [ModInt998244353::new(0); 2];
        for (i, ab) in ab.iter().copied().enumerate() {
            for (j, cd) in pre.iter().copied().enumerate() {
                if ab != cd {
                    arr2[i] += arr[j];
                }
            }
        }
        arr = arr2;
        pre = ab;
    }
    let rs = (arr[0] + arr[1]) / 2;
    println!("{rs}");
}
