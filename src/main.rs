use std::fmt::{Debug, Display};

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

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn new(width: u32, height: u32) -> Self {
//         Rectangle { width, height }
//     }
// }

// enum Shape {
//     Circle,
//     Square(u32),
//     Triangle { base: u32, height: u32 },
// }

// impl Shape {
//     fn sample_method(&self) {
//         println!("sample_method")
//     }
// }

fn max<T>(a: T, b: T) -> T
where
    T: PartialOrd + Debug,
{
    if a >= b {
        a
    } else {
        b
    }
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T: PartialOrd + Debug> Point<T> {
    fn max(&self) -> &T {
        if self.x >= self.y {
            &self.x
        } else {
            &self.y
        }
    }

    fn print_arg<U: Display>(&self, val: U) {
        println!("self.x: {:?}", self.x);
        println!("val: {}", val);
    }
}

fn need_even(a: i32) -> Result<i32, String> {
    if a % 2 == 0 {
        Ok(a)
    } else {
        Err(String::from("引数は偶数にしてください。"))
    }
}

fn double_even(b: i32) -> Result<i32, String> {
    let x: i32 = need_even(b)?;
    Ok(x * 2)
}

// use rust_hello_world::sample_trait::{double_area, Circle, Rectangle, Shape};
fn main() {
    match double_even(1) {
        Ok(val) => println!("{}", val),
        Err(err) => {
            println!("mainでハンドリング");
            println!("{}", err);
        }
    }
}
