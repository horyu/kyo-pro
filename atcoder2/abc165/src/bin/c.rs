#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings, map_first_last)]
use itertools::{iproduct, izip, Itertools as _};
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
        abcds: [(usize, usize, usize, usize); q],
    };
    let mut max = 0;
    let mut nums = vec![1; n];
    while {
        let sum = calc(&nums, &abcds);
        if sum > max {
            max = sum;
        }
        next(&mut nums, &m)
    } {}
    println!("{}", max);
}

fn calc(nums: &[usize], abcds: &[(usize, usize, usize, usize)]) -> usize {
    let mut sum = 0;
    for &(a, b, c, d) in abcds {
        if nums[b - 1] - nums[a - 1] == c {
            sum += d;
        }
    }
    sum
}

fn next(nums: &mut Vec<usize>, m: &usize) -> bool {
    // 右から9のindexを探す
    // ある → 一番左にあるmの左隣をインクリメントし右を全部それに
    // ない → 右端を+1
    let len = nums.len();
    let ii = nums.iter().position(|x| x == m);
    if let Some(i) = ii {
        if i == 0 {
            return false;
        } else {
            nums[i - 1] += 1;
            for j in i..len {
                nums[j] = nums[i - 1];
            }
        }
    } else {
        nums[len - 1] += 1;
    }
    true
}
