
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("someone1@example.com");
    println!("User 1: {}", user1.username);
    println!("User 1 email: {}", user1.email);
    println!("User 1 active: {}", user1.active);
    println!("User 1 sign_in_count: {}", user1.sign_in_count);

    let user2 = build_user(String::from("hello@gmail.com"), String::from("hello"));
    println!("Build user: {}", user2.username);

    let user3 = build_user_shortand_syntax(String::from("shortand@gmail.com"), String::from("shortand"));
    println!("Build user shortand: {}", user3.username);

    let user4 = create_user_from_another();
    println!("Create user from another: {}", user4.username);

    struct_tuple();

    unit_struct();

    concrete_example();
}

/*
 * Defining and Instantiating Structs
 */

 struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn build_user_shortand_syntax(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn create_user_from_another() -> User {
    let user1 = User {
        active: true,
        username: String::from("user"),
        email: String::from("user1@email.com"),
        sign_in_count: 1
    };
    let user2 = User {
        active: false,
        ..user1 // must be the last field
    };
    // println!("User 1: {}", user1.username); ERROR user1.username is not available because it is borrowed by user2
    user2
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn struct_tuple() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Tuple struct black: {}, {}, {}", black.0, black.1, black.2);
    println!("Tuple struct origin: {}, {}, {}", origin.0, origin.1, origin.2);
}

struct AlwaysEqual;

fn unit_struct() {
    let subject = AlwaysEqual;
}

// ownership of struct data: https://doc.rust-lang.org/stable/book/ch05-01-defining-structs.html#ownership-of-struct-data


/*
 * Concrete example
 */

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// area function is outside of the Rectangle struct
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
// area function (method) is inside of the Rectangle struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn increase(&mut self, width: u32, height: u32) {
        self.width += width;
        self.height += height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn create(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn concrete_example() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!("rect1 with {{:#?}}: {:#?}", rect1);
    println!("rect1 with {{:?}}: {:?}", rect1);
    // USE reference because dbg! macro is borrowing the parameter
    dbg!(&rect1);
    println!("react area from the method: {}", rect1.area());

    let mut rect2 = Rectangle {
        width: 30,
        height: 30,
    };
    println!("rect2 before increase: {:#?}", rect2);
    rect2.increase(10, 10);
    println!("rect2 after increase: {:#?}", rect2);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    println!("Create square: {:#?}", Rectangle::create(10));
}

