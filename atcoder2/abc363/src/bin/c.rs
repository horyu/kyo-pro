#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use permutohedron::LexicalPermutation;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        k: usize,
        mut s: Chars,
    };
    s.sort_unstable();
    let mut rs = 0;
    loop {
        if (0..=(n - k)).all(|mut l| {
            let mut r = l + k - 1;
            while l < r {
                if s[l] != s[r] {
                    return true;
                }
                l += 1;
                r -= 1;
            }
            false
        }) {
            rs += 1;
        }
        if !s.next_permutation() {
            break;
        }
    }

    println!("{rs}");
}
