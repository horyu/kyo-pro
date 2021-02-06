#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        _n: usize,
        k: usize,
        r: usize,
        s: usize,
        p: usize,
        t: Chars,
    };
    let mut sum = 0;
    // [0,k,2k,...], [1,k+1,2k+1,...], [...] 分けて考える
    for start in 0..k {
        let mut pre = '_';
        for c in t.iter().skip(start).step_by(k) {
            if c == &pre {
                pre = '_';
                continue;
            }
            sum += match c {
                'r' => p,
                's' => r,
                'p' => s,
                _ => unreachable!(),
            };
            pre = *c;
        }
    }
    println!("{}", sum);
}
