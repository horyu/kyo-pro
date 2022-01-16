#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{fastout, input, marker::*};
use std::collections::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        hh: [usize; n]
    };
    let mut rs = hh[0];
    for (&hi, &hj) in hh.iter().tuple_windows() {
        if (rs == hi) && hi < hj {
            rs = hj;
        } else {
            break;
        }
    }
    println!("{}", rs);
}
