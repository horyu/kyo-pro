#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        xxyy: [(isize, isize); n]
    };
    let tf = xxyy.iter().combinations(3).any(|abc| {
        let (ax, ay) = abc[0];
        let (bx, by) = abc[1];
        let (cx, cy) = abc[2];
        // c を原点に
        let (px, py) = (ax - cx, ay - cy);
        let (qx, qy) = (bx - cx, by - cy);
        if (px == 0) || (qx == 0) {
            return (px == 0) && (qx == 0);
        }
        if (py == 0) || (qy == 0) {
            return (py == 0) && (qy == 0);
        }
        px * qy == py * qx
    });
    println!("{}", ["No", "Yes"][tf as usize]);
}
