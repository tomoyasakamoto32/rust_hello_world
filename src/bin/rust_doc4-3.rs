fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);
    let word2 = seconds_word(&s);

    println!("{},{}", word, word2);
    let x = &s;
    println!("{}", x);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..2];

    println!("{:?}", a);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn seconds_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i + 1..];
        }
    }

    &s[..]
}
