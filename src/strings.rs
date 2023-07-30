use std::io;

fn strings() {
    let x = 5;
    println!("The x = {x}");

    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';

    println!("{c}, {z}, {heart_eyed_cat}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("{x}, {y}, {z}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let f500 = x.0;

    println!("Five Hundred : {f500}");

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Please enter an array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is {element}");

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let jan = months[0];
    println!("{jan}")
}
