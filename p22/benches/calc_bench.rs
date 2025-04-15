#![feature(test)] // Enable the unstable test feature

extern crate test; // Import the test crate
use p22::calc::{fibonacci_loop, fibonacci_rec};
use test::{Bencher, black_box};

#[bench]
fn bench_fibonacci_loop(b: &mut Bencher) {
    let n = black_box(10); // Use black_box to prevent the compiler from optimizing away the function call
    b.iter(|| fibonacci_loop(n));
}

#[bench]
fn bench_fibonacci_rec(b: &mut Bencher) {
    let n = black_box(10);
    b.iter(|| fibonacci_rec(n));
}
