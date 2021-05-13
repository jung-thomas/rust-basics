const MAX_POINTS: u32 = 100_000;
use std::io;

fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
    println!("The value of x is: {}", MAX_POINTS);

    // let mut spaces = "   ";
    // spaces = spaces.len();

    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;

    // remainder
    let _remainder = 43 % 5;

    let _t = true;

    let _f: bool = false; // with explicit type annotation

    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x, y, z is: {}, {}, {}", x, y, z);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = x.0;
    let _six_point_four = x.1;
    let _one = x.2;

    let _a = [1, 2, 3, 4, 5];

    let _months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];   
    
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let _a = [3; 5];

    let a = [1, 2, 3, 4, 5];
    let _first = a[0];
    let _second = a[1];

    invalid_array()
}

fn invalid_array(){
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
