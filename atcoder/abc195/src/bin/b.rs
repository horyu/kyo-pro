#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        a: usize,
        b: usize,
        w: usize,
    };
    let w = w * 1000;
    // a <= w/(w/b + 1) <= b は
    let mut min = w / b;
    if w % b == 0 {
    } else if (a * (w / b + 1) <= w) && (w <= b * (w / b + 1)) {
        min += 1;
    } else {
        println!("UNSATISFIABLE");
        return;
    }
    let mut max = w / a;
    // 減らさない a + (w % a)/(w / a) <= b は
    // 1個減らす a <= w/(w/a - 1) <= b は
    if w % a == 0 {
    } else if a as f64 + (w % a) as f64 / (w / a) as f64 <= b as f64 {
    } else if (a * (w / a - 1) <= w) && (w <= b * (w / a - 1)) {
        max -= 1;
    } else {
        println!("UNSATISFIABLE");
        return;
    }
    println!("{} {}", min, max);
}
