trait Shape {
    fn area(&self) -> u32;
    fn perimeter(&self) -> u32;
}

struct Rectangle {
    x: u32,
    y: u32,
}

impl Rectangle {
    fn new(x: u32, y: u32) -> Rectangle {
        Rectangle { x, y }
    }
}

impl Shape for Rectangle {
    fn area(&self) -> u32 {
        self.x * self.y
    }

    fn perimeter(&self) -> u32 {
        2 * (self.x + self.y)
    }
}

struct Square {
    x: u32,
}

impl Square {
    fn new(x: u32) -> Square {
        Square { x }
    }
}

impl Shape for Square {
    fn area(&self) -> u32 {
        self.x + self.x
    }

    fn perimeter(&self) -> u32 {
        4 * self.x
    }
}

pub fn iterator() {
    let r = Rectangle::new(3, 5);
    let s = Square::new(5);
    let shapes: Vec<&dyn Shape> = vec![
        &r,
        &s,
    ];

    for (i, shape) in shapes.iter().enumerate() {
        println!("Shape {} area is {}", i, shape.area());
        println!("Shape {} perimeter is {}", i, shape.perimeter());
    }
}
