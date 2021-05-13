fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("hello");
    change(&mut s2);

    let mut s3 = String::from("hello");

    {
        let _r1 = &mut s3;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let _r2 = &mut s3;


    let r3 = &s3; // no problem
    let r4 = &s3; // no problem
    println!("{} and {}", r3, r4);
    // r1 and r2 are no longer used after this point

    let r5 = &mut s3; // no problem
    println!("{}", r5);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
