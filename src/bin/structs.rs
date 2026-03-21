//Unit struct
// for traits
struct AlwaysEqual;

//colors
struct Color(i32, i32, i32);

struct User {
    email: String,
    phone_number: String,
    active: bool
}

struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn area(&self) -> u32 {
        return self.height * self.width
    }

    fn square(size: u32) -> Self {
        Rectangle { width: size, height: size }
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
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // constructing a square
    let square = Rectangle::square(3);
}
