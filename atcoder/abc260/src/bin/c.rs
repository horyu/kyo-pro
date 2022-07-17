#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;
 
fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
    };
    if n == 1 {
        println!("0");
        return;
    }
    let mut aa = vec![0usize; n];
    aa[n - 1] = 1;
    let mut bb = vec![0usize; n];
    while 1 < aa.len() {
        let a = aa.pop().unwrap();
        *aa.last_mut().unwrap() += a;
        *bb.last_mut().unwrap() += a * x;
        let b = bb.pop().unwrap();
        *aa.last_mut().unwrap() += b;
        *bb.last_mut().unwrap() += b * y;
    }
    println!("{}", bb[0]);
}