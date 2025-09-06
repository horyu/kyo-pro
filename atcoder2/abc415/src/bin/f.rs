#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![allow(dead_code)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use num_traits::WrappingSub;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[cfg(not(debug_assertions))]
macro_rules! eprintln {
    ($($tt:tt)*) => {};
}
fn main() {
    input! {
        n: usize,
        q: usize,
        mut s: Chars,
    };
    // https://atcoder.jp/contests/abc415/submissions/67806684
    use ac_library::{Monoid, Segtree};
    #[derive(Clone, Copy, Debug)]
    struct S {
        lc: char,
        ln: usize,
        rc: char,
        rn: usize,
        max: usize,
        size: usize,
    }
    impl Monoid for S {
        type S = Option<S>;
        fn identity() -> Self::S {
            None
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            match (a, b) {
                (_, None) => *a,
                (None, _) => *b,
                (Some(a), Some(b)) => {
                    let size = a.size + b.size;
                    if a.lc == b.rc && a.ln == a.size && b.rn == b.size {
                        Some(S {
                            lc: a.lc,
                            ln: size,
                            rc: b.rc,
                            rn: size,
                            max: size,
                            size,
                        })
                    } else {
                        Some(S {
                            lc: a.lc,
                            ln: if a.rc == b.lc && a.ln == a.size {
                                a.ln + b.ln
                            } else {
                                a.ln
                            },
                            rc: b.rc,
                            rn: if a.rc == b.lc && b.ln == b.size {
                                a.rn + b.ln
                            } else {
                                b.rn
                            },
                            max: std::cmp::max(
                                a.max.max(b.max),
                                if a.rc == b.lc { a.rn + b.ln } else { 0 },
                            ),
                            size,
                        })
                    }
                }
            }
        }
    }

    let mut st = Segtree::<S>::new(n);
    for (i, s) in s.into_iter().enumerate() {
        st.set(
            i,
            Some(S {
                lc: s,
                ln: 1,
                rc: s,
                rn: 1,
                max: 1,
                size: 1,
            }),
        );
    }
    for _ in 0..q {
        input! {t: usize};
        if t == 1 {
            input! {i: Usize1, x: char};
            st.set(
                i,
                Some(S {
                    lc: x,
                    ln: 1,
                    rc: x,
                    rn: 1,
                    max: 1,
                    size: 1,
                }),
            );
            continue;
        }
        input! {l: Usize1, r: Usize1};
        let s = st.prod(l..=r).unwrap();
        let rs = s.max;
        println!("{rs}");
    }
}

fn main2() {
    input! {
        n: usize,
        q: usize,
        mut s: Chars,
    };
    use ac_library::{LazySegtree, MapMonoid, Max};
    struct MaxMonoid;
    impl MapMonoid for MaxMonoid {
        type M = Max<usize>;
        type F = usize;

        fn identity_map() -> Self::F {
            0
        }

        fn mapping(&f: &usize, &x: &usize) -> usize {
            f.max(x)
        }

        fn composition(&f: &usize, &g: &usize) -> usize {
            f.max(g)
        }
    }
    let mut btm = BTreeMap::new();
    for (_c, g) in s.iter().copied().group_by(|&c| c).into_iter() {
        let len = g.count();
        if let Some((&prev_idx, &prev_len)) = btm.last_key_value() {
            btm.insert(prev_idx + prev_len, len);
        } else {
            btm.insert(0, len);
        }
    }
    let mut ls = ac_library::LazySegtree::<MaxMonoid>::new(n);
    for (&k, &v) in btm.iter() {
        ls.set(k, v);
    }
    eprintln!(
        "{}\n{} {btm:?}",
        s.iter().join(""),
        (0..n).map(|i| ls.get(i)).join("")
    );
    for _ in 0..q {
        input! {t: usize};
        if t == 1 {
            input! {i: Usize1, x: char};
            // 重複
            if s[i] == x {
                continue;
            }
            s[i] = x;
            // ..i 開始のグループがiと重なる場合に事前に分割
            if let Some((&idx, &len)) = btm.range(..i).next_back() {
                // aaaa:(0,4) i=2 x=b
                // aaba:(0,2),(2,1),(3,1)
                if i == idx + len - 1 {
                    // iが右端
                    btm.insert(idx, len - 1);
                    ls.set(idx, len - 1);
                } else if i < idx + len - 1 {
                    // iが中間: (idx..=(i-1)),i..=i,((i+1)..=(idx+len-1))
                    // 長さ   : i-idx, 1, idx+len-i-1
                    let l_len = i - idx;
                    btm.insert(idx, l_len);
                    ls.set(idx, l_len);
                    let r_len = (idx + len) - (i + 1);
                    btm.insert(i + 1, r_len);
                    ls.set(i + 1, r_len);
                }
                // btm.insert(i, 1);
                // ls.set(i, 1);
            }
            // i.. の範囲を更新
            if let Some(len) = btm.remove(&i)
                && 1 < len
            {
                // i開始のグループを縮小
                btm.insert(i + 1, len - 1);
                ls.set(i + 1, len - 1);
            }
            // 一旦 i を更新
            btm.insert(i, 1);
            ls.set(i, 1);
            if s.get(i + 1) == Some(&x) {
                // i, i+1 のグループを結合
                let len = btm.remove(&(i + 1)).unwrap();
                ls.set(i + 1, 0);
                btm.insert(i, len + 1);
                ls.set(i, len + 1);
            }
            // ..=i の範囲を更新
            if s.get(i.wrapping_sub(1)) == Some(&x) {
                // 結合
                let l_idx = *btm.range(..i).next_back().unwrap().0;
                let l_len = btm.remove(&l_idx).unwrap();
                let r_len = btm.remove(&i).unwrap();
                ls.set(i, 0);
                btm.insert(l_idx, l_len + r_len);
                ls.set(l_idx, l_len + r_len);
            }
            eprintln!(
                "{}\n{} {btm:?}",
                s.iter().join(""),
                (0..n).map(|i| ls.get(i)).join("")
            );
            continue;
        }
        input! {mut l: Usize1, mut r: Usize1};
        let mut rs = 1;
        // ..lのグループが重なる場合
        if let Some((&idx, &len)) = btm.range(..l).next_back()
            && l < idx + len
        {
            // l..=(r.min(idx + len - 1))
            rs = rs.max(r.min(idx + len - 1) - l + 1);
            l = idx + len;
        }
        // ..rのグループが飛び出る場合
        if l <= r
            && let Some((&idx, &len)) = btm.range(l..=r).next_back()
            && r < idx + len - 1
        {
            rs = rs.max(r - idx + 1);
            r = idx.saturating_sub(1);
        }
        if l < r {
            rs = rs.max(ls.prod(l..=r));
        }
        println!("{rs}");
    }
}
