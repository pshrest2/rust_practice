fn main() {
    let object = Rectangle {
        length: 10,
        breadth: 10
    };
    let area = object.area();
    println!("The length is {}", object.length);
    println!("The breadth is {}", object.breadth);
    println!("Therefore, the area is {area}");
    println!("object is {:#?}", object);
    dbg!(&object);
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
}