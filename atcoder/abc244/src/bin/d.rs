#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        s1: char,
        s2: char,
        s3: char,
        t1: char,
        t2: char,
        t3: char,
    };
    let tf = (s1 == t1 && s2 == t2 && s3 == t3)
        || (s1 == t3 && s2 == t1 && s3 == t2)
        || (s1 == t2 && s2 == t3 && s3 == t1);

    println!("{}", ["No", "Yes"][tf as usize]);
}
