use scratch::ownership::take_ownership;

fn main() {
    // println!("Hello, world!");
    let s = String::from("hello");
    println!("{}", take_ownership(s));
}
