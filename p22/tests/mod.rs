use p22::calc::{celsius2farenheit, farenheit2celsius, fibonacci_loop, fibonacci_rec};
use p22::figures::Circle;
use p22::figures::Point;
use p22::figures::Rectangle;
use p22::figures::Triangle;
use p22::figures::perimeter_cercle;
use p22::figures::perimeter_rectangle;
use p22::figures::perimeter_triangle;

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

#[test]
fn test_perimeter_triangle() {
    let a = Point::new(0.0, 0.0);
    let b = Point::new(3.0, 0.0);
    let c = Point::new(3.0, 4.0);
    let t = Triangle::new(a, b, c);
    let perimeter = perimeter_triangle(t);
    assert_eq!(perimeter, 12.0);
}
#[test]
fn test_perimeter_cercle() {
    let a = Point::new(0.0, 0.0);
    let c = Circle::new(a, 3.0);
    let perimeter = perimeter_cercle(c);
    assert_eq!(perimeter, 6.0 * std::f64::consts::PI);
}
#[test]
fn test_perimeter_rectangle() {
    let a = Point::new(0.0, 0.0);
    let b = Point::new(3.0, 4.0);
    let r = Rectangle::new(a, b);
    let perimeter = perimeter_rectangle(r);
    assert_eq!(perimeter, 14.0);
}
