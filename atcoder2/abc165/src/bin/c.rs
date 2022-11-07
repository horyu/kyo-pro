#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
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
        q: usize,
        aabbccdd: [(Usize1, Usize1, usize, usize); q],
    };
    let mut rs = 0;
    let mut nums = vec![1; n];
    loop {
        let mut sum = 0;
        for &(a, b, c, d) in &aabbccdd {
            if nums[a] + c == nums[b] {
                sum += d;
            }
        }
        rs = rs.max(sum);

        // 右からmのindexを探す
        // ある → 一番左にあるmの左隣をインクリメントし右を全部それに
        // ない → 右端を+1
        let len = nums.len();
        if let Some(i) = nums.iter().position(|&x| x == m) {
            if i == 0 {
                break;
            } else {
                nums[i - 1] += 1;
                for j in i..len {
                    nums[j] = nums[i - 1];
                }
            }
        } else {
            nums[len - 1] += 1;
        }
    }
    println!("{}", rs);
}
