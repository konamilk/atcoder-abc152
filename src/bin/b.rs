use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

fn main() {
    // let source = AutoSource::from("0");
    input!{
        // from source,
        mut a: [usize; 2]
    };

    a.sort();

    let mut ans = String::new();

    for _ in 0..a[1]{
        ans = format!("{}{}", ans, a[0])
    }

    println!("{}", ans);
}
