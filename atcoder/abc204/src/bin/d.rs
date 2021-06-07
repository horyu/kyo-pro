#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        tt: [usize; n]
    };
    if n <= 2 {
        println!("{}", tt.iter().max().unwrap());
        return;
    }
    // https://www.slideshare.net/chokudai/arc029
    // https://qiita.com/drken/items/a5e6fe22863b7992efdb#%E5%95%8F%E9%A1%8C-3%E9%83%A8%E5%88%86%E5%92%8C%E5%95%8F%E9%A1%8C
    let m = tt.iter().sum::<usize>();
    let mut dp = vec![vec![false; m + 1]; n + 1];
    dp[0][0] = true;
    for i in 0..n {
        for j in 0..=m {
            dp[i + 1][j] |= dp[i][j];
            if j >= tt[i] {
                dp[i + 1][j] |= dp[i][j - tt[i]];
            }
        }
    }
    for rs in (m.div_ceil(&2))..m {
        if dp[n][rs] {
            println!("{}", rs);
            return;
        }
    }
}
