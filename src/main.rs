#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    let rect2 = Rectangle {
        width: 15,
        height: 44
    };

    let rect3 = Rectangle {
        width: 45,
        height: 57,
    };

    let square = Rectangle::square(88);

    println!("rect: {:#?}", rect1);
    println!("The area of rectangle is {}", rect1.area());

    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3: {}", rect1.can_hold(&rect3));
    println!("rect2 can hold rect1: {}", rect2.can_hold(&rect1));
    println!("rect2 can hold rect3: {}", rect2.can_hold(&rect3));
    println!("rect3 can hold rect1: {}", rect3.can_hold(&rect1));
    println!("rect3 can hold rect2: {}", rect3.can_hold(&rect2));
    println!("square can hold rect2: {}", square.can_hold(&rect1));
    println!("square can hold rect2: {}", square.can_hold(&rect2));
    println!("square can hold rect2: {}", square.can_hold(&rect3));
}
