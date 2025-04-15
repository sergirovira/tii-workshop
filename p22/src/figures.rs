#[derive(Default, Debug, Clone, Copy)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.x == other.x {
            if self.y == other.y {
                std::cmp::Ordering::Equal
            } else if self.y < other.y {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        } else if self.x < other.x {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }
}
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Point {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Point {
    fn distance(&self, other: &Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Circle {
    center: Point,
    radius: f64,
}

impl Circle {
    pub fn new(center: Point, radius: f64) -> Self {
        Self { center, radius }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}

impl Triangle {
    pub fn new(a: Point, b: Point, c: Point) -> Self {
        Self { a, b, c }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Rectangle {
    a: Point,
    b: Point,
}

impl Rectangle {
    pub fn new(a: Point, b: Point) -> Self {
        Self { a, b }
    }
}

pub enum Shape {
    Cercle(Circle),
    Triangle(Triangle),
    Rectangle(Rectangle),
}

// compute area of triangle using implementation of point outside of the struct
/// Example:
/// ```
/// use p22::figures::{Point, area_triangle};
/// let a = Point::new(0.0, 0.0);
/// let b = Point::new(3.0, 0.0);
/// let c = Point::new(3.0, 4.0);
/// let area = area_triangle(a, b, c);
/// assert_eq!(area, 6.0);
/// ```
pub fn area_triangle(a: Point, b: Point, c: Point) -> f64 {
    let ab = a.distance(&b);
    let bc = b.distance(&c);
    let ca = c.distance(&a);
    let s = (ab + bc + ca) / 2.0;
    (s * (s - ab) * (s - bc) * (s - ca)).sqrt()
}

/// Example:    
/// ```
/// use p22::figures::{Point, Circle, area_cercle};
/// let a = Point::new(0.0, 0.0);
/// let c = Circle::new(a, 3.0);
/// let area = area_cercle(c);
/// assert_eq!(area, std::f64::consts::PI * 9.0);
/// ```
pub fn area_cercle(c: Circle) -> f64 {
    std::f64::consts::PI * c.radius.powi(2)
}

/// Example:
/// ```
/// use p22::figures::{Point, Rectangle, area_rectangle};
/// let a = Point::new(0.0, 0.0);
/// let b = Point::new(3.0, 4.0);
/// let r = Rectangle::new(a, b);
/// let area = area_rectangle(r);
/// assert_eq!(area, 12.0);
/// ```
pub fn area_rectangle(r: Rectangle) -> f64 {
    let a = r.a.x - r.b.x;
    let b = r.a.y - r.b.y;
    let a = a.abs();
    let b = b.abs();
    a * b
}

/// Example:
/// ```
/// use p22::figures::{Point, Rectangle, area_shape, Shape};
/// let a = Point::new(0.0, 0.0);
/// let b = Point::new(3.0, 4.0);
/// let r = Rectangle::new(a, b);
/// let area = area_shape(Shape::Rectangle(r));
/// assert_eq!(area, 12.0);
/// ```
pub fn area_shape(s: Shape) -> f64 {
    match s {
        Shape::Cercle(c) => area_cercle(c),
        Shape::Triangle(t) => area_triangle(t.a, t.b, t.c),
        Shape::Rectangle(r) => area_rectangle(r),
    }
}

pub fn perimeter_cercle(c: Circle) -> f64 {
    2.0 * std::f64::consts::PI * c.radius
}
pub fn perimeter_triangle(t: Triangle) -> f64 {
    t.a.distance(&t.b) + t.b.distance(&t.c) + t.c.distance(&t.a)
}
pub fn perimeter_rectangle(r: Rectangle) -> f64 {
    let a = r.a.x - r.b.x;
    let b = r.a.y - r.b.y;
    let a = a.abs();
    let b = b.abs();
    2.0 * (a + b)
}
pub fn perimeter_shape(s: Shape) -> f64 {
    match s {
        Shape::Cercle(c) => perimeter_cercle(c),
        Shape::Triangle(t) => perimeter_triangle(t),
        Shape::Rectangle(r) => perimeter_rectangle(r),
    }
}

#[test]
fn test_area_triangle() {
    let a = Point::new(0.0, 0.0);
    let b = Point::new(3.0, 0.0);
    let c = Point::new(3.0, 4.0);
    let area = area_triangle(a, b, c);
    assert_eq!(area, 6.0);
}
#[test]
fn test_area_cercle() {
    let a = Point::new(0.0, 0.0);
    let c = Circle::new(a, 3.0);
    let area = area_cercle(c);
    assert_eq!(area, std::f64::consts::PI * 9.0);
}
#[test]
fn test_area_rectangle() {
    let a = Point::new(0.0, 0.0);
    let b = Point::new(3.0, 4.0);
    let r = Rectangle::new(a, b);
    let area = area_rectangle(r);
    assert_eq!(area, 12.0);
}
