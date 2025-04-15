use p22::calc::{celsius2farenheit, farenheit2celsius, fibonacci_loop, fibonacci_rec};

#[test]
fn test_celsius2farenheit() {
    let celsius = 0;
    let farenheit = celsius2farenheit(celsius);
    assert_eq!(farenheit, 32);
}

#[test]
fn test_farenheit2celsius() {
    let farenheit = 32;
    let celsius = farenheit2celsius(farenheit);
    assert_eq!(celsius, 0);
}

#[test]
fn test_fibonacci_loop() {
    let n = 10;
    let fib = fibonacci_loop(n);
    assert_eq!(fib, 55);
}

#[test]
fn test_fibonacci_rec() {
    let n = 10;
    let fib = fibonacci_rec(n);
    assert_eq!(fib, 55);
}
