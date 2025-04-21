// This module contains the calculator functions

///This function converts Celsius to Fahrenheit
/// Example
/// ```
/// use p22::calc::celsius2farenheit;
/// let celsius = 0;
/// let farenheit = celsius2farenheit(celsius);
/// assert_eq!(farenheit, 32);
/// ```
pub fn celsius2farenheit(celsius: i32) -> i32 {
    (celsius * 9 / 5) + 32
}

///This function converts Fahrenheit to Celsius
/// Example
/// ```
/// use p22::calc::farenheit2celsius;
/// let farenheit = 32;
/// let celsius = farenheit2celsius(farenheit);
/// assert_eq!(celsius, 0);
/// ```
/// ```
/// use p22::calc::farenheit2celsius;
/// let farenheit = 100;
/// let celsius = farenheit2celsius(farenheit);
/// assert_eq!(celsius, 37);
/// ```
pub fn farenheit2celsius(farenheit: i32) -> i32 {
    (farenheit - 32) * 5 / 9
}

///This function calculates the nth Fibonacci number
/// Example
/// ```
/// use p22::calc::fibonacci_loop;
/// let n = 10;
/// let fib = fibonacci_loop(n);
/// assert_eq!(fib, 55);
/// ```
pub fn fibonacci_loop(n: u32) -> u64 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let temp = a;
        a = b;
        b += temp;
    }
    a
}

///This function calculates the nth Fibonacci number
/// Example
/// ```
/// use p22::calc::fibonacci_rec;
/// let n = 10;
/// let fib = fibonacci_rec(n);
/// assert_eq!(fib, 55);
/// ```
pub fn fibonacci_rec(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_rec(n - 1) + fibonacci_rec(n - 2),
    }
}
