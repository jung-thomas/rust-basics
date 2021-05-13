struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {

    fn square(size: u32) -> Rectangle {
        Rectangle{
            width: size,
            height: size
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    user_stuff();

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

    println!("rect1 is {:?}", rect1);
    //let rect1 = (30,50);
    //let width1 = 30;
    //let height1 = 50;

    println!(
        "The area of the regtangle is {} square pixels",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let _sq = Rectangle::square(3);
}

/* fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
} */

fn user_stuff() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    let _user2 = build_user(String::from("Test@test.com"), String::from("myuser"));

    let _user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
