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
        h: usize,
        w: usize,
        aaa: [[usize; w]; h],
    };
    let hw = h * w;
    let mut rs = 0;
    for bits in 0usize..(1 << hw) {
        if bits.count_ones() % 2 != 0 {
            continue;
        }
        // (i,j)&(i,j+1) or (i,j)&(i+1,j) で全て埋められるかチェック
        fn dfs(bits: usize, h: usize, w: usize) -> bool {
            if bits == 0 {
                return true;
            }
            for i in 0..h {
                for j in 0..w {
                    let ij = i * w + j;
                    if bits & (1 << ij) == 0 {
                        continue;
                    }
                    // (i,j)と(i,j+1)が両方埋められるか
                    if j + 1 < w && bits & (1 << (ij + 1)) != 0 {
                        let new_bits = bits ^ (1 << ij) ^ (1 << (ij + 1));
                        if dfs(new_bits, h, w) {
                            return true;
                        }
                    }
                    // (i,j)と(i+1,j+1)が両方埋められるか
                    if i + 1 < h && bits & (1 << (ij + w)) != 0 {
                        let new_bits = bits ^ (1 << ij) ^ (1 << (ij + w));
                        if dfs(new_bits, h, w) {
                            return true;
                        }
                    }
                    return false; // どちらも埋められない場合はfalse
                }
            }
            false
        }
        if dfs(bits, h, w) {
            let mut tmp = 0;
            for (i, aa) in aaa.iter().enumerate() {
                for (j, a) in aa.iter().copied().enumerate() {
                    if bits & (1 << (i * w + j)) == 0 {
                        tmp ^= a;
                    }
                }
            }
            rs = rs.max(tmp);
        }
    }
    println!("{rs}");
}
