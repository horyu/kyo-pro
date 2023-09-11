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
        n: usize,
        m: usize,
    };
    // https://github.com/E869120/kyopro_educational_90/blob/main/sol/057.cpp
    let mut mat = vec![vec![0u8; m]; n];
    for i in 0..n {
        input! {
            t: usize,
            aa: [Usize1; t],
        };
        for a in aa {
            mat[i][a] = 1;
        }
    }
    input! {
        mut ss: [u8; m],
    };
    let mut pos = 0;
    for i in 0..m {
        let mut found = false;
        for j in pos..n {
            if mat[j][i] == 1 {
                if j != pos {
                    mat.swap(j, pos);
                }
                found = true;
                break;
            }
        }
        if found {
            for j in 0..n {
                if j != pos && mat[j][i] == 1 {
                    for k in i..m {
                        mat[j][k] ^= mat[pos][k];
                    }
                }
            }
            if ss[i] == 1 {
                for k in i..m {
                    ss[k] ^= mat[pos][k];
                }
            }
            pos += 1;
        }
    }
    if ss == vec![0; m] {
        let mut rs = ModInt998244353::new(1);
        for i in pos..n {
            rs *= 2;
        }
        println!("{rs}");
    } else {
        println!("0");
    }
}
