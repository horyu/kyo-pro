#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
    };
    // https://atcoder.jp/contests/abc254/editorial/4079
    let pf = PrimeFact::new(n);
    let mut vv = vec![0usize; n + 1];
    vv[1] = 1;
    for i in 2..=n {
        let lpf = pf.least_factor(i);
        if vv[i / lpf] % lpf == 0 {
            vv[i] = vv[i / lpf] / lpf;
        } else {
            vv[i] = vv[i / lpf] * lpf;
        }
    }
    let mut ww = vec![0usize; n + 1];
    for &v in &vv[1..]  {
        ww[v] += 1;
    }
    let rs = ww.into_iter().fold(0, |acc, w| acc + w.pow(2u32));
    println!("{rs}");
}


// https://algo-logic.info/prime-fact/
struct PrimeFact {
    spf: Vec<usize>,
}
#[allow(dead_code)]
impl PrimeFact {
    fn new(n: usize) -> Self {
        let mut spf = (0..=n).collect_vec();
        for i in (2..=n).take_while(|x| x * x <= n) {
            if spf[i] == i {
                for j in (i..=n).step_by(i) {
                    if spf[j] == j {
                        spf[j] = i;
                    }
                }
            }
        }
        Self { spf }
    }

    fn get(&self, mut n: usize) -> HashMap<usize, usize> {
        let mut hm = HashMap::new();
        while n != 1 {
            *hm.entry(self.spf[n]).or_insert(0) += 1usize;
            n /= self.spf[n];
        }
        hm
    }

    fn least_factor(&self, n: usize) -> usize {
        self.spf[n]
    }
}
