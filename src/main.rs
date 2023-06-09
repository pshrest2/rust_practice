fn main() {
    let rect1 = Rectangle { length: 10, breadth: 10 };
    let rect2 = Rectangle { length: 10, breadth: 10 };
    let rect3 = Rectangle { length: 15, breadth: 20 };
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3: {}", rect1.can_hold(&rect3));

    let square1 = Rectangle::square(10);
    println!("Area of square is: {}", square1.area());
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    breadth: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.breadth
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        let area_self = self.area();
        let area_rect = rectangle.area();

        area_self >= area_rect
    }

    fn square(size: u32) -> Self {
        Self { length:  size, breadth: size }
    }
}