fn main() {
    let mut s = String::new();
    let data = "initial contents";
    let s1 = data.to_string();
    let s2 = "initial contents".to_string();
    let s3 = String::from("initial contents");
    hello();

    let mut s4 = String::from("foo");
    s4.push_str("bar");

    let mut s5 = String::from("foo");
    let s6 = "bar";
    s5.push_str(s6);
    println!("s2 is {}, but s1 is {}", s6, s5 );

    let mut s7 = String::from("lo");
    s7.push('l');

    let s8 = String::from("Hello, ");
    let s9 = String::from("world!");
    let s10 = s8 + &s9; // note s8 has been moved here and can no longer be used

    let s11 = String::from("tic");
    let s12 = String::from("tac");
    let s13 = String::from("toe");    
    let s14 = format!("{}-{}-{}", s1, s2, s3);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}

fn hello() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}
