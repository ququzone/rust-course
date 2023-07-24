use std::fmt::Debug;
use std::ops::Add;

trait Printable {
    fn stringify(&self) -> String;
}

#[derive(Debug, Clone)]
struct Bag<T>
where
    T: ToString + Add,
{
    size: T,
}

impl<T: ToString + Add<Output = T>> Add for Bag<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            size: self.size + rhs.size,
        }
    }
}

impl<T: ToString + Add<Output = T>> Printable for Bag<T> {
    fn stringify(&self) -> String {
        format!("{}{}", "Bag:", self.size.to_string())
    }
}

fn print_bag(b: &dyn Printable) {
    println!("{}", b.stringify());
}

pub fn demo() {
    let a: Bag<u32> = Bag { size: 3 };
    let b: Bag<u32> = Bag { size: 4 };

    println!("bag(a + b): {:?}", a.clone() + b.clone());
    print_bag(&a);
}
