pub fn check_value(x: i32) {
    if x > 5 {
        println!("big");
    } else {
        println!("small");
    }
}

pub fn none() {
    for i in 0..5 {
        println!("{}", i);
    }
}

struct User {
    name: String,
    age: u32,
}

fn main() {
    let user = User {
        name: String::from("Abel"),
        age: 21,
    };
    println!("{}{}", user.name, user.age);
}
