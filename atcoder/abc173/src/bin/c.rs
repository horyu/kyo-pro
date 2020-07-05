#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        h: usize,
        w: usize,
        k: i32,
        ccc: [Chars; h]
    };
    let mut ok = 0;
    for row_size in 0..h {
        for row_indexes in (0..h).combinations(row_size) {
            for col_size in 0..w {
                for col_indexes in (0..w).combinations(col_size) {
                    let mut count = 0;
                    for row in 0..h {
                        for col in 0..w {
                            if !row_indexes.contains(&row)
                                && !col_indexes.contains(&col)
                                && (ccc[row][col] == '#')
                            {
                                count += 1;
                            }
                        }
                    }
                    if count == k {
                        ok += 1;
                    }
                }
            }
        }
    }
    println!("{}", ok);
}
