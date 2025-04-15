#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct point {
    x: f64,
    y: f64,
}

#[derive(Default, Debug, Clone, Copy)]
struct cercle {
    center: point,
    radius: f64,
}

#[derive(Default, Debug, Clone, Copy)]
struct triangel {
    a: point,
    b: point,
    c: point,
}

#[derive(Default, Debug, Clone, Copy)]
struct rectangle {
    a: point,
    b: point,
}

enum shape {
    cercle(cercle),
    triangel(triangel),
    rectangle(rectangle),
}
