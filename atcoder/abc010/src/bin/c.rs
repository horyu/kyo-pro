#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        txa : f64,
        tya : f64,
        txb : f64,
        tyb : f64,
        t: f64,
        v: f64,
        n: usize,
        xxyy: [(f64, f64); n]
    };
    for &(x, y) in xxyy.iter() {
        let distance = ((txa - x).powf(2.) + (tya - y).powf(2.)).sqrt()
            + ((txb - x).powf(2.) + (tyb - y).powf(2.)).sqrt();
        if distance <= t * v {
            println!("YES");
            return;
        }
    }
    println!("NO");
}
