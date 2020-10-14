#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        h: usize,
        w: usize,
        sss: [Chars; h]
    };
    for y in 0..h {
        for x in 0..w {
            if !ok_urdl(&sss, y, x, h, w) {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}

fn ok_urdl(sss: &[Vec<char>], i: usize, j: usize, h: usize, w: usize) -> bool {
    if sss[i][j] == '.' {
        return true;
    }
    return (i > 0 && sss[i - 1][j] == '#')
        || (i < h - 1 && sss[i + 1][j] == '#')
        || (j > 0 && sss[i][j - 1] == '#')
        || (j < w - 1 && sss[i][j + 1] == '#');
}
