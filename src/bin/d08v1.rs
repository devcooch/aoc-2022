use std::cmp::max;

fn main() {
    let a: Vec<Vec<_>> = include_str!("test.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();
    let n = a.len();
    let m = a[0].len();
    for i in 0..n {
        println!("{:?}", a[i]);
    }
    println!("");
    let mut l2r = vec![vec![0i32; m]; n];
    let mut r2l = vec![vec![0i32; m]; n];
    let mut t2b = vec![vec![0i32; m]; n];
    let mut b2t = vec![vec![0i32; m]; n];
    for i in 0..n {
        let mut x = 0u32;
        for j in 0..m {
            x = max(x, a[i][j]);
            l2r[i][j] = x;
        }
    }
    for i in 0..n {
        println!("{:?}", l2r[i]);
    }
    println!("");
    for i in 0..n {
        let mut x = 0u32;
        for j in (0..m).rev() {
            x = max(x, a[i][j]);
            r2l[i][j] = x;
        }
    }
    for i in 0..n {
        println!("{:?}", r2l[i]);
    }
    println!("");
    for j in 0..m {
        let mut x = 0u32;
        for i in 0..n {
            x = max(x, a[i][j]);
            t2b[i][j] = x;
        }
    }
    for i in 0..n {
        println!("{:?}", t2b[i]);
    }
    println!("");
    for j in 0..m {
        let mut x = 0u32;
        for i in (0..n).rev() {
            x = max(x, a[i][j]);
            b2t[i][j] = x;
        }
    }
    for i in 0..n {
        println!("{:?}", b2t[i]);
    }
    println!("");
    let mut v: Vec<Vec<u32>> = vec![vec![0u32; m]; n];
    for i in 0..n {
        for j in 0..m {
            let x = a[i][j];
            v[i][j] =
                ((l2r[i][j] < x) || (r2l[i][j] < x) || (t2b[i][j] < x) || (b2t[i][j] < x)) as u32;
        }
    }
    for i in 0..n {
        println!("{:?}", v[i]);
    }
    /*
    let s: u32 = a
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, x)| {
                    (l2r[i][j] < *x || r2l[i][j] < *x || t2b[i][j] < *x || b2t[i][j] < *x)
                        as u32
                })
                .sum::<u32>()
        })
        .sum();
    println!("{}", s);
    */
}
