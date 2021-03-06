#![allow(dead_code)]
pub struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)] // let you print {:?}
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    // mutable, suspicious code
    fn area_moved(self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    // mutable struct
    let mut user1 = User {
        email: String::from("contact@karlk.im"),
        username: String::from("karlkim"),
        active: true,
        sign_in_count: 1,
    };
    user1.sign_in_count = 2;
    println!("Hello, {}", user1.username);

    let user2 = build_user(String::from("hi@karlk.im"), String::from("kk"));
    println!("Hello, {}", user2.username);

    let user3 = User {
        email: String::from("hello@karlk.im"),
        username: String::from("k8s"),
        ..user1
    };

    println!("Hello, {}", user3.username);

    try_tuple_structs();
    try_rectangles();
}

fn try_rectangles() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
    println!("rect1 area(&self) is {}", rect1.area());
    let _x = rect1.height;
    println!("rect1 area_moved(self) is {}", rect1.area_moved());
    // height moved, below doesn't compile
    // let _y = rect1.height;

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

    let sq = Rectangle::square(3);
    println!("sq is {:#?}", sq);
}

fn try_tuple_structs() {
    // Tuple Struct
    struct Color(i32, i32, i32);
    #[derive(Debug)] // let you print {:?}
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let end_point: Point = Point(std::i32::MAX, std::i32::MAX, std::i32::MAX);

    let Color(b1, b2, b3) = black; // destructuring Tuple Struct
    println!("Black is {}, {}, {}", b1, b2, b3);
    println!("Origin is {:?}", origin);
    println!(
        "End Point is {}, {}, {}",
        end_point.0, end_point.1, end_point.2
    );
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
