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
    let source = AutoSource::from("25");
    input!{
        // from source,
        n: u64
    };

    let mut tbl = vec![vec![0i64;10];10];

    for i in 1..=n{
        if i % 10 == 0 {
            continue
        }
        // if reverse_num(i) > n {
        //     continue
        // }
        let bottom = i % 10;
        let mut top = 0;
        let mut j = i;
        while j > 0{
            top = j % 10;
            j = j / 10;
        }

        tbl[top as usize][bottom as usize]+=1;
    }

    let mut ans = 0i64;

    for i in 1..=9{
        for j in 1..=9{
            ans += tbl[i][j] * tbl[j][i]
        }
    }

    println!("{}", ans);
}

pub fn reverse_num(mut n: u64) -> u64{
    let mut v = vec![];
    while n > 0{
        v.push(n % 10);
        n = n / 10;
    }

    v.iter().fold(0, |acc, x| acc * 10 + x)
}

#[test]
fn test_reverse_num() {
    assert_eq!(reverse_num(1), 1);
    assert_eq!(reverse_num(1234), 4321);
    assert_eq!(reverse_num(20000), 2);
}
