fn main() {
    let x = add(1,5);
    println!("Hello, world! {:?}",x);
}

fn add(a:i32,b:i32) -> i32 {
    a+b
}