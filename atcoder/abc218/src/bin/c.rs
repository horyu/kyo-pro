#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::{
    collections::{HashMap, HashSet},
    vec,
};

fn main() {
    input! {
        n: isize,
        mut sss: [Chars; n],
        ttt: [Chars; n],
    };
    if sss
        .iter()
        .map(|ss| ss.iter().filter(|&&s| s == '#').count())
        .sum::<usize>()
        != ttt
            .iter()
            .map(|tt| tt.iter().filter(|&&t| t == '#').count())
            .sum::<usize>()
    {
        println!("No");
        return;
    }
    for _ in 0..4 {
        if is_same(&sss, &ttt, n) {
            println!("Yes");
            return;
        }
        let mut new_sss = vec![vec!['.'; n as usize]; n as usize];
        for (i, ss) in sss.iter().enumerate() {
            for (j, s) in ss.iter().enumerate() {
                new_sss[n as usize - 1 - j][i] = *s;
            }
        }
        sss = new_sss;
    }
    println!("No");
}

fn is_same(sss: &[Vec<char>], ttt: &[Vec<char>], n: isize) -> bool {
    let (si, sj) = find_left_top(sss);
    let (ti, tj) = find_left_top(ttt);
    let offset_i = ti - si;
    let offset_j = tj - sj;
    for i in 0..n {
        for j in 0..n {
            let ii = i + offset_i;
            let jj = j + offset_j;
            if (0..n).contains(&ii) && (0..n).contains(&jj) {
                if sss[i as usize][j as usize] != ttt[ii as usize][jj as usize] {
                    return false;
                }
            } else if sss[i as usize][j as usize] == '#' {
                return false;
            }
        }
    }
    true
}

fn find_left_top(ggg: &[Vec<char>]) -> (isize, isize) {
    for (i, gg) in ggg.iter().enumerate() {
        for (j, g) in gg.iter().enumerate() {
            if *g == '#' {
                return (i as isize, j as isize);
            }
        }
    }
    unreachable!();
}
