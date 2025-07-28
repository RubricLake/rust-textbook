#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// Methods go here
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other : &Rectangle) -> bool {
        self.area() >= other.area()
    }

    // Constructor-Like
    fn square(size : u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let square1 = Rectangle::square(100);
    dbg!(square1);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); // True
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3)); // False
}