#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library::ModInt998244353;
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        t: usize,
        nnss: [(usize, Bytes); t],
    };
    for (n, s) in nnss {
        let mut rs = ModInt998244353::default();
        let m = n.div_ceil(2);
        for i in 0..m {
            let kouho = s[i] - b'A';
            rs += ModInt998244353::new(26).pow((m - 1 - i) as u64) * kouho;
        }
        // abc  n3 i0
        // abcd n4 i01
        let mut tf = true;
        for i in 0..m {
            // eprintln!("{n} {i} {} {}", s[m - 1 - i] as char, s[n / 2 + i] as char);
            match s[m - 1 - i].cmp(&s[n / 2 + i]) {
                std::cmp::Ordering::Less => break,
                std::cmp::Ordering::Equal => continue,
                std::cmp::Ordering::Greater => {
                    tf = false;
                    break;
                }
            }
        }
        rs += tf as u32;
        println!("{rs}");
    }
}
