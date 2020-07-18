#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let f = s.len();
    let mut ans = 2 * (f - 1);
    if f == 3 {
        println!("7");
        return;
    }
    for i in 1..f - 1 {
        if s[i] == 'U' {
            ans += (f - i - 1) + i * 2;
        } else {
            ans += (f - i - 1) * 2 + i;
        }
    }
    println!("{}", ans);
}
