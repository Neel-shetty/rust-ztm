fn main() {
    enums();
}

enum Direction {
    Left,
    Right,
    Up
}

fn enums(){
    let go = Direction::Up;
    match go {
        Direction::Left => println!("left"),
        Direction::Right => println!("Right"),
        Direction::Up => println!("up")
    }
}

fn loops(){
    let mut a = 1;
    loop {
        if a == 5 {
            break;
        }
        println!("{:?}",a);
        a+=1;
    }
}

fn matching(){
    let a = 1;
    match a {
        5 => println!("its 5"),
        6 => println!("its 6"),
        _ => println!("its idk")
    }
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