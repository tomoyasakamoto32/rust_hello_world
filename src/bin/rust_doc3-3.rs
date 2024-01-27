fn main() {
    println!("Hello, world!");

    another_function(21, '円');
}

fn another_function(x: i32, unit_label: char) {
    println!("The measurement is: {} {}", x, unit_label);
}
