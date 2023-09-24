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
        s: Bytes,
    };
    let mut qq = vec![];
    let mut ttff = [false; 256];
    let mut btm = BTreeMap::new();
    for (i, c) in s.iter().copied().enumerate() {
        match c {
            b'(' => {
                qq.push(i);
            }
            b')' => {
                let j = qq.pop().unwrap();
                while let Some((&k, &v)) = btm.range(j..i).next() {
                    ttff[v as usize] = false;
                    btm.remove(&k);
                }
            }
            c => {
                if ttff[c as usize] {
                    println!("No");
                    return;
                }
                ttff[c as usize] = true;
                btm.insert(i, c);
            }
        }
    }
    println!("Yes");
}
