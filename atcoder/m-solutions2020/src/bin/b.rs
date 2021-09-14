#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
        mut c: usize,
        mut k: usize
    };
    while k > 0 {
        k -= 1;
        if a >= b {
            b *= 2;
        } else if b >= c {
            c *= 2;
        } else {
            break;
        }
    }
    println!("{}", ["No", "Yes"][((a < b) && (b < c)) as usize]);
}
