#![allow(unused_imports)]
use std::collections::HashMap;
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut ss: [String; n]
    };
    ss.reverse();
    let mut memo: HashMap<(bool, usize), usize> = HashMap::new();
    println!("{}", calc(true, &ss[..], &mut memo));
}

fn calc(tf: bool, ops: &[String], memo: &mut HashMap<(bool, usize), usize>) -> usize {
    let len = ops.len();
    if let Some(&rs) = memo.get(&(tf, len)) {
        return rs;
    }

    let op = ops[0].as_str();
    let rs = if len == 1 {
        match (tf, op) {
            (true, "AND") => 1,
            (false, "AND") => 3,
            (true, "OR") => 3,
            (false, "OR") => 1,
            _ => unreachable!(),
        }
    } else {
        match (tf, op) {
            (true, "AND") => calc(true, &ops[1..], memo),
            (false, "AND") => calc(true, &ops[1..], memo) + 2 * calc(false, &ops[1..], memo),
            (true, "OR") => 2 * calc(true, &ops[1..], memo) + calc(false, &ops[1..], memo),
            (false, "OR") => calc(false, &ops[1..], memo),
            _ => unreachable!(),
        }
    };
    memo.insert((tf, len), rs);
    rs
}
