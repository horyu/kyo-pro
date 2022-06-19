#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
#![feature(let_chains)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
    };
    if n == 1 {
        println!("1");
        return;
    }
    let nn = n * n;
    let mut rs = (1..=nn).collect_vec();
    let mut swaped = vec![false; nn];
    let make_jj = |i: usize| -> Vec<usize> {
        let iy = i / n;
        let ix = i % n;
        let mut jj = Vec::new();
        if iy > 0 {
            if ix > 0 {
                jj.push((iy - 1) * n + ix - 1);
            }
            jj.push((iy - 1) * n + ix);
            if ix < n - 1 {
                jj.push((iy - 1) * n + ix + 1);
            }
        }
        if ix > 0 {
            jj.push(i - 1);
        }
        if ix < n - 1 {
            jj.push(i + 1);
        }
        if iy < n - 1 {
            if ix > 0 {
                jj.push((iy + 1) * n + ix - 1);
            }
            jj.push((iy + 1) * n + ix);
            if ix < n - 1 {
                jj.push((iy + 1) * n + ix + 1);
            }
        }
        jj
    };
    let is_ok = |i: usize, jj: &Vec<usize>, rs: &Vec<usize>| -> bool {
        let mut dd = [0, 0];
        for &j in jj {
            dd[(rs[i] < rs[j]) as usize] += 1;
        }
        dd[0] != dd[1]
    };
    for y in 1..(n - 1) {
        for x in 1..(n - 1) {
            let i = y * n + x;
            let jj = make_jj(i);
            if !is_ok(i, &jj, &rs) {
                swaped[i] = true;
                if !swaped[i - 1] {
                    rs.swap(i - 1, i);
                    swaped[i - 1] = true;
                } else {
                    rs.swap(i, i + 1);
                    swaped[i + 1] = true;
                }
            }
        }
    }
    // for (i, &v) in rs.iter().enumerate() {
    // let jj = make_jj(i);
    // if !is_ok(i, &jj, &rs) {
    //     eprintln!("{i} {v} [{}]:[{}]", jj.iter().join(","), jj.iter().map(|&j| rs[j]).join(","));
    //     // panic!();
    // }
    // dbg!(i, v, dd[0] != dd[1]);
    // }
    for i in 0..n {
        println!("{}", &rs[i * n..(i + 1) * n].iter().join(" "));
    }
}
