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
        aaa: [[isize; 3]; 3],
    };
    // https://atcoder.jp/contests/abc349/editorial/9780
    let mut s = [-1; 9];
    fn check(s: &[isize; 9]) -> isize {
        for p in 0..3 {
            // 横
            if s[p * 3] != -1 && (0..3).map(|q| s[p * 3 + q]).all_equal() {
                return s[p * 3];
            }
            // 縦
            if s[p] != -1 && (0..3).map(|q| s[p + q * 3]).all_equal() {
                return s[p];
            }
        }
        // 斜め
        if s[0] != -1 && (0..3).map(|i| s[i * 3 + i]).all_equal() {
            return s[0];
        }
        if s[2] != -1 && (0..3).map(|i| s[i * 3 + 2 - i]).all_equal() {
            return s[2];
        }
        -1
    }
    fn f(aaa: &Vec<Vec<isize>>, s: &mut [isize; 9], turn: usize) -> usize {
        let c = check(s);
        if c != -1 {
            return c as usize;
        }
        // スコア計算
        if !s.contains(&-1) {
            let mut scores = [0; 2];
            for x in 0..3 {
                for y in 0..3 {
                    scores[s[x * 3 + y] as usize] += aaa[x][y];
                }
            }
            return scores.into_iter().position_max().unwrap();
        }
        for k in 0..9 {
            if s[k] == -1 {
                s[k] = turn as isize;
                if f(aaa, s, turn ^ 1) == turn {
                    s[k] = -1;
                    return turn;
                }
                s[k] = -1;
            }
        }

        turn ^ 1
    }
    let rs = ["Takahashi", "Aoki"][f(&aaa, &mut s, 0)];
    println!("{rs}");
}
