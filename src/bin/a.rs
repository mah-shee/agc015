#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }
    if a > b {
        println!("0");
    } else if a == b {
        println!("1");
    } else {
        if n == 1 && a != b {
            println!("0");
        } else {
            let ans = (b - a) * (n - 2) + 1;
            println!("{}", ans);
        }
    }
}
