trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

struct Rectangle {
    x: f64,
    y: f64,
}

impl Rectangle {
    fn new(x: f64, y: f64) -> Rectangle {
        Rectangle { x, y }
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.x * self.y
    }

    fn perimeter(&self) -> f64 {
        2f64 * (self.x + self.y)
    }
}

struct Square {
    x: f64,
}

impl Square {
    fn new(x: f64) -> Square {
        Square { x }
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.x + self.x
    }

    fn perimeter(&self) -> f64 {
        4f64 * self.x
    }
}

struct Circle {
    r: f64
}

impl Circle {
    fn new(r: f64) -> Circle {
        Circle { r }
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14 * self.r * self.r
    }

    fn perimeter(&self) -> f64 {
        3.14 * 2f64 * self.r
    }
}

pub fn iterator() {
    let r = Rectangle::new(3f64, 5f64);
    let s = Square::new(5f64);
    let c = Circle::new(3f64);
    let shapes: Vec<&dyn Shape> = vec![
        &r,
        &s,
        &c,
    ];

    for (i, shape) in shapes.iter().enumerate() {
        println!("Shape {} area is {}", i, shape.area());
        println!("Shape {} perimeter is {}", i, shape.perimeter());
    }
}
