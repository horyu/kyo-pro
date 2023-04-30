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
        h: usize,
        w: usize,
        a: usize,
        b: usize,
    };
    let rs = dfs(&mut vec![vec![0; w]; h], a, b);
    println!("{rs}");
}

fn dfs(mat: &mut Vec<Vec<u8>>, a: usize, b: usize) -> usize {
    if a == 0 && b == 0 {
        // eprintln!("[{a},{b}] {} {}", mat[0].iter().join(" "), mat[1].iter().join(" "));
        return 1;
    }
    let h = mat.len();
    let w = mat[0].len();
    for i in 0..h {
        for j in 0..w {
            if mat[i][j] == 0 {
                let mut cnt = 0;
                if 0 < a {
                    if i + 1 < h && mat[i + 1][j] == 0 {
                        mat[i][j] = 1;
                        mat[i + 1][j] = 1;
                        // eprintln!("![{a},{b}] {} {}", mat[0].iter().join(" "), mat[1].iter().join(" "));
                        cnt += dfs(mat, a - 1, b);
                        mat[i][j] = 0;
                        mat[i + 1][j] = 0;
                    }
                    if j + 1 < w && mat[i][j + 1] == 0 {
                        mat[i][j] = 2;
                        mat[i][j + 1] = 2;
                        cnt += dfs(mat, a - 1, b);
                        mat[i][j] = 0;
                        mat[i][j + 1] = 0;
                    }
                }
                if 0 < b {
                    mat[i][j] = 3;
                    // eprintln!("?[{a},{b}] {} {}", mat[0].iter().join(" "), mat[1].iter().join(" "));
                    cnt += dfs(mat, a, b - 1);
                    mat[i][j] = 0;
                }
                return cnt;
            }
        }
    }
    0
}
