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
        s: [Chars; h],
    };
    let snuke_vec = vec!['s', 'n', 'u', 'k', 'e'];
    for i in 0..=(h - 5) {
        for j in 0..w {
            if (0..5).map(|k| s[i + k][j]).collect_vec() == snuke_vec {
                for k in 0..5 {
                    println!("{} {}", i + 1 + k, j + 1);
                }
                return;
            }
            if (0..5).map(|k| s[i + 4 - k][j]).collect_vec() == snuke_vec {
                for k in 0..5 {
                    println!("{} {}", i + 5 - k, j + 1);
                }
                return;
            }
        }
    }
    for j in 0..=(w - 5) {
        for i in 0..h {
            if (0..5).map(|k| s[i][j + k]).collect_vec() == snuke_vec {
                for k in 0..5 {
                    println!("{} {}", i + 1, j + 1 + k);
                }
                return;
            }
            if (0..5).map(|k| s[i][j + 4 - k]).collect_vec() == snuke_vec {
                for k in 0..5 {
                    println!("{} {}", i + 1, j + 5 - k);
                }
                return;
            }
        }
    }
    for i in 0..h {
        for j in 0..w {
            if i + 4 < h && j + 4 < w {
                if (0..5).map(|k| s[i + k][j + k]).collect_vec() == snuke_vec {
                    for k in 0..5 {
                        println!("{} {}", i + 1 + k, j + 1 + k);
                    }
                    return;
                }
                if (0..5).map(|k| s[i + 4 - k][j + k]).collect_vec() == snuke_vec {
                    for k in 0..5 {
                        println!("{} {}", i + 5 - k, j + 1 + k);
                    }
                    return;
                }
                if (0..5).map(|k| s[i + k][j + 4 - k]).collect_vec() == snuke_vec {
                    for k in 0..5 {
                        println!("{} {}", i + 1 + k, j + 5 - k);
                    }
                    return;
                }
                if (0..5).map(|k| s[i + 4 - k][j + 4 - k]).collect_vec() == snuke_vec {
                    for k in 0..5 {
                        println!("{} {}", i + 5 - k, j + 5 - k);
                    }
                    return;
                }
            }
        }
    }
}
