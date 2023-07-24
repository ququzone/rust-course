struct Rectangle {
    x: u32,
    y: u32,
}

struct Square {
    x: u32,
}

struct Circle {
    r: u32,
}

impl Rectangle {
    fn new(x: u32, y: u32) -> Rectangle {
        Rectangle { x, y }
    }

    fn area(&self) -> u32 {
        self.x * self.y
    }

    fn perimeter(&self) -> u32 {
        2 * (self.x + self.y)
    }
}

impl Square {
    fn new(x: u32) -> Square {
        Square { x }
    }

    fn area(&self) -> u32 {
        self.x * self.x
    }

    fn perimeter(&self) -> u32 {
        4 * self.x
    }
}

impl Circle {
    fn new(r: u32) -> Circle {
        Circle { r }
    }

    fn area(&self) -> f64 {
        3.14 * (self.r * self.r) as f64
    }

    fn perimeter(&self) -> f64 {
        3.14 * (2 * self.r) as f64
    }
}

pub fn iterator() {
    enum VecShape {
        R(Rectangle),
        S(Square),
        C(Circle),
    }

    let shapes: Vec<VecShape> = vec![
        VecShape::R(Rectangle::new(2, 3)),
        VecShape::S(Square::new(3)),
        VecShape::C(Circle::new(3)),
    ];

    for shape in shapes.iter() {
        match shape {
            VecShape::R(r) => {
                println!("Rectangle area is {}", r.area());
                println!("Rectangle perimeter is {}", r.perimeter());
            }
            VecShape::S(s) => {
                println!("Square area is {}", s.area());
                println!("Square perimeter is {}", s.perimeter());
            }
            VecShape::C(c) => {
                println!("Circle area is {}", c.area());
                println!("Circle perimeter is {}", c.perimeter());
            }
        }
    }
}
