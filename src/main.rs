fn print_value(s: &String) {
    println!("{}",s);
}

fn main() {
    let s = String::from("hello")
    print_value(&s);
}
