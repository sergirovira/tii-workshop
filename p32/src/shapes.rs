trait Shape<const NAME: usize> {
    fn perimeter(&self) -> f64;
    fn area(&self) -> f64;
    fn scale(&mut self, factor: f64);
    fn area_to_perimeter(&self) -> f64;
    fn bigguest_area(&self, other: &Self) -> f64;
    fn point_properties(&self) -> Vec<(f64, f64)>;
}

enum DynamicShape {
    Circle(Circle),
    Triangle(Triangle),
    Rectangle(Rectangle),
}

struct Circle {
    radius: f64,
}

struct Triangle {
    base: f64,
    height: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Point {
    x: f64,
    y: f64,
}
impl Shape<2> for Triangle {
    fn perimeter(&self) -> f64 {
        self.base + 2.0 * (self.base.powi(2) + self.height.powi(2)).sqrt()
    }

    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }

    fn scale(&mut self, factor: f64) {
        self.base *= factor;
        self.height *= factor;
    }

    fn area_to_perimeter(&self) -> f64 {
        0.5 * self.area() / self.perimeter()
    }

    fn bigguest_area(&self, other: &Self) -> f64 {
        if self.area() > other.area() {
            self.area()
        } else {
            other.area()
        }
    }

    fn point_properties(&self) -> Vec<(f64, f64)> {
        vec![(0.0, 0.0), (self.base, 0.0), (self.base / 2.0, self.height)]
    }
}
impl Shape<3> for Rectangle {
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }

    fn area_to_perimeter(&self) -> f64 {
        0.5 * self.area() / self.perimeter()
    }

    fn bigguest_area(&self, other: &Self) -> f64 {
        if self.area() > other.area() {
            self.area()
        } else {
            other.area()
        }
    }

    fn point_properties(&self) -> Vec<(f64, f64)> {
        vec![
            (0.0, 0.0),
            (self.width, 0.0),
            (self.width, self.height),
            (0.0, self.height),
        ]
    }
}

impl Shape<1> for Circle {
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }

    fn scale(&mut self, factor: f64) {
        self.radius *= factor;
    }

    fn area_to_perimeter(&self) -> f64 {
        0.5 * self.area() / self.perimeter()
    }

    fn bigguest_area(&self, other: &Self) -> f64 {
        if self.area() > other.area() {
            self.area()
        } else {
            other.area()
        }
    }

    fn point_properties(&self) -> Vec<(f64, f64)> {
        vec![(0.0, 0.0)]
    }
}

fn find_biggest_ratio<'a>(
    shapes1: &'a [DynamicShape],
    shapes2: &'a [DynamicShape],
) -> Option<&'a DynamicShape> {
    let mut biggest_ratio = 0.0;
    let mut biggest_shape = None;

    for shape in shapes1.iter().chain(shapes2.iter()) {
        let ratio = match shape {
            DynamicShape::Circle(circle) => circle.perimeter() / circle.area(),
            DynamicShape::Triangle(triangle) => triangle.perimeter() / triangle.area(),
            DynamicShape::Rectangle(rectangle) => rectangle.perimeter() / rectangle.area(),
        };

        if ratio > biggest_ratio {
            biggest_ratio = ratio;
            biggest_shape = Some(shape);
        }
    }

    biggest_shape
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle() {
        let mut circle = Circle { radius: 5.0 };
        assert_eq!(circle.perimeter(), 31.41592653589793);
        assert_eq!(circle.area(), 78.53981633974483);
        circle.scale(2.0);
        assert_eq!(circle.radius, 10.0);
        assert_eq!(circle.area_to_perimeter(), 2.5);
    }

    #[test]
    fn test_triangle() {
        let mut triangle = Triangle {
            base: 3.0,
            height: 4.0,
        };
        assert_eq!(triangle.perimeter(), 13.0);
        assert_eq!(triangle.area(), 6.0);
        triangle.scale(2.0);
        assert_eq!(triangle.base, 6.0);
        assert_eq!(triangle.height, 8.0);
        assert_eq!(triangle.area_to_perimeter(), 0.5);
    }

    #[test]
    fn test_rectangle() {
        let mut rectangle = Rectangle {
            width: 4.0,
            height: 5.0,
        };
        assert_eq!(rectangle.perimeter(), 18.0);
        assert_eq!(rectangle.area(), 20.0);
        rectangle.scale(2.0);
        assert_eq!(rectangle.width, 8.0);
        assert_eq!(rectangle.height, 10.0);
        assert_eq!(rectangle.area_to_perimeter(), 1.1111111111111112);
    }
    #[test]
    fn test_find_biggest_ratio() {
        let circle = DynamicShape::Circle(Circle { radius: 5.0 });
        let triangle = DynamicShape::Triangle(Triangle {
            base: 3.0,
            height: 4.0,
        });
        let rectangle = DynamicShape::Rectangle(Rectangle {
            width: 4.0,
            height: 5.0,
        });

        let shapes1 = vec![circle, triangle];
        let shapes2 = vec![rectangle];

        let biggest_shape = find_biggest_ratio(&shapes1, &shapes2);
        assert!(biggest_shape.is_some());
    }
}
