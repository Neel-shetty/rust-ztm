fn main() {
    compare_numbers();
}

fn compare_numbers(){
    let a = 5;
    if a < 5 {
        println!("Number is lesser than 5")
    } else if a > 5 {
        println!("Number is greater than 5")
    } else {
        println!("Number is 5")
    }
}

fn boolean() {
    let x = false;
    if x==true {
        println!("Hello, world! {:?}",x);
    } else {
        println!("Byee")
    }
}

fn add(a:i32,b:i32) -> i32 {
    a+b
}