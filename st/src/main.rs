struct User {
    username: String,
    email: String,
    sing_in_count: u64,
    active: bool
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32
}


impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
}

fn main() {
    let user = User {
        email: String::from("ksin@naver.com"),
        username: String::from("username"),
        active: true,
        sing_in_count: 1
    };

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    let rect1 = Rectangle { length: 50 , width: 30 };

    println!("rect is {:?}",rect1);
    println!("{}", rect1.area())

}


fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active:true,
        sing_in_count: 1
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}