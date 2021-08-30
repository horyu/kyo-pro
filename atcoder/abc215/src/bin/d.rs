#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        m: usize,
        aa: [usize; n]
    };
    const SIZE: usize = 100_001usize;
    let mut vv = vec![true; SIZE];
    for a in aa {
        for i in trial_division(a) {
            if vv[i] {
                for j in (i..SIZE).step_by(i) {
                    vv[j] = false;
                }
            }
        }
    }
    let rs = (1..=m)
        .filter(|i| vv[*i])
        .map(|i| i.to_string())
        .collect_vec();
    println!("{}\n{}", rs.len(), rs.join("\n"));
}

// https://qiita.com/Cassin01/items/2f90aedded2b8fb017a1#%E8%A9%A6%E3%81%97%E5%89%B2%E3%82%8A
fn trial_division(mut n: usize) -> Vec<usize> {
    let mut primes = Vec::new();
    let mut i = 2;

    //  n を root n 以下の整数で割り切れるまで割っていく
    while i * i <= n {
        while n % i == 0 {
            n /= i;
            primes.push(i);
        }
        i += 1;
    }
    // 最後にnが素数になっている場合はそれ自身も素因数に含めて終了
    if n > 1 {
        primes.push(n);
    }
    primes
}
