fn fizzbuzz1(end: i32) {
    let mut cnt: i32 = 1;
    while cnt <= end {
        if cnt % 3 == 0 && cnt % 5 == 0 {
            println!("FizzBuzz");
        } else if cnt % 3 == 0 {
            println!("Fizz");
        } else if cnt % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", cnt);
        }
        cnt += 1;
    }
}

fn fizzbuzz2(end: i32) {
    for i in 1..=end {
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", i),
        }
    }
}

fn main() {
    println!("Hello World");
}
