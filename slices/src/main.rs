fn main() {
    let s = String::from("hello world");
    let word = first_word(&s); // word will get the value 5
                               //s.clear(); // this empties the String, making it equal to ""
    println!("The first word is: {}", word);

    let my_string_literal = "hello world";
    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);
    println!("The first word is: {}", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("The first word is: {}", word);
    let s2 = String::from("hello world");

    let _hello = &s2[0..5];
    let _world = &s2[6..11];
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
