fn main() {
    let _v: Vec<i32> = Vec::new();
    let _v2 = vec![1, 2, 3];

    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    let v4 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v4[2];
    println!("The third element is {}", third);

    match v4.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    let v4 = vec![100, 32, 57];
    for i in &v4 {
        println!("{}", i)
    }

    let mut v5 = vec![100, 32, 57];
    for i in &mut v5 {
        *i += 50;
    }
    for i in &v5 {
        println!("{}", i)
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        println!("{:?}", i)
    }
}
