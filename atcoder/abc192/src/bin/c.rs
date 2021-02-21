#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut k: usize,
    };
    let mut a = n;

    while (k > 0) && (a > 0) {
        k -= 1;
        a = f(a);
    }
    println!("{}", a);
}

fn f(x: usize) -> usize {
    g1(x) - g2(x)
}

fn g1(x: usize) -> usize {
    let v = num_to_vec(x);
    vec_to_num(v)
}

fn g2(x: usize) -> usize {
    let mut v = num_to_vec(x);
    v.reverse();
    while let Some(0) = v.first() {
        v.remove(0);
    }
    vec_to_num(v)
}

fn num_to_vec(mut x: usize) -> Vec<usize> {
    let mut v = vec![];
    if x == 0 {
        return v;
    }
    while x > 0 {
        v.push(x % 10);
        x /= 10;
    }
    v.sort_unstable();
    v
}

fn vec_to_num(v: Vec<usize>) -> usize {
    v.iter()
        .enumerate()
        .fold(0, |acc, (i, num)| acc + num * 10usize.pow(i as u32))
}
