//! ライブラリクレートのドキュメント

/// say_hello関数のドキュメント
pub fn say_hello() {
    println!("This is rust-lesson");
}

pub mod sample_trait {
    pub trait Shape {
        fn calc_area(&self) -> f64;
        fn calc_perimeter(&self) -> f64;
        fn default_something(&self) -> &str {
            "default"
        }
        fn do_something();
    }

    pub struct Rectangle {
        pub width: f64,
        pub height: f64,
    }

    impl Shape for Rectangle {
        fn calc_area(&self) -> f64 {
            self.width * self.height
        }

        fn calc_perimeter(&self) -> f64 {
            self.width * 2.0 + self.height * 2.0
        }

        fn default_something(&self) -> &str {
            "rect default"
        }

        fn do_something() {
            println!("This is Rectangle function")
        }
    }

    pub struct Circle {
        pub radius: f64,
    }

    impl Shape for Circle {
        fn calc_area(&self) -> f64 {
            self.radius * self.radius * std::f64::consts::PI
        }

        fn calc_perimeter(&self) -> f64 {
            self.radius * 2.0 * std::f64::consts::PI
        }

        fn do_something() {
            println!("This is Circle function")
        }
    }

    pub fn double_area(shape: &impl Shape) -> f64 {
        shape.calc_area() * 2.0
    }
}
