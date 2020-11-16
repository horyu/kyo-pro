#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut xx: [isize; m]
    };
    if n >= m || m == 1 {
        println!("0");
        return;
    }
    xx.sort();
    if n == 1 {
        println!("{}", xx[n - 1] - xx[0]);
        return;
    }
    // windows(2)で前から調べる
    // 距離が2以上ある地点を調べ、距離が一番離れてる塊ごとにコマを置く
    // コマをおいたら距離1の地点を消してまた↑を実行
    // 距離2の塊がなくなったら余ったコマの数だけカウントから引く

    // println!("{}",);
}
