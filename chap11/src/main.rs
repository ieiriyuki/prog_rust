use std::ops::{Add, Mul};

fn main() {
    println!("Hello, world!");

    println!("{}", dot(&[1, 2, 3], &[4, 5, 6]));
    println!("{}", dot(&[1.0, 2.2, 3.0], &[4., 5., 6.]));
}

// 11.5
fn dot<N>(v1: &[N], v2: &[N]) -> N
    where N: Add<Output = N> + Mul<Output = N> + Default + Copy
{
    let mut total = N::default();
    for i in 0..v1.len() {
        total = total + v1[i] * v2[i]
    }
    total
}
