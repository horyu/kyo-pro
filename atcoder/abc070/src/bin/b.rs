#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;

fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
        d: isize,
    };
    let kaisi = max(a, c);
    let owari = min(b, d);
    println!("{}", if kaisi < owari { owari - kaisi } else { 0 });
}
