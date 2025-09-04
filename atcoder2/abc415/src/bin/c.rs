#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        t: usize,
        nnss: [(usize, Chars); t],
    };
    // for (i, &c) in nnss[0].1.iter().enumerate() {
    //     let i = i + 1;
    //     eprintln!("[{i}] {i:03b}: {}", if  c == '0' { 'o' } else { 'x' });
    // }
    'outer: for (n, mut s) in nnss {
        let mut qq = VecDeque::new();
        qq.push_back(0);
        let ok = (1usize << n) - 1;
        while let Some(i) = qq.pop_front() {
            // if n == 3 {
            //     eprintln!("{i:03b}");
            // }
            for j in 0..n {
                let ii = i | (1 << j);
                if ii == i || s[ii - 1] == '1' {
                    continue;
                }
                if ii == ok {
                    println!("Yes");
                    continue 'outer;
                }
                qq.push_back(ii);
                s[ii - 1] = '1';
            }
        }
        println!("No");
    }
}
