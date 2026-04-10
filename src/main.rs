fn print_value(s: &String) {
    println!("{}",s);
}

fn main() {
    let s = String::from("hello");
    print_value(&s);
}

fn add(a:i32, b:i32) -> i32 {
    a + b
}
