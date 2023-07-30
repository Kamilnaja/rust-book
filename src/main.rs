fn main() {
    // let x = five();
    // println!("The value of the x is {x}");

    // let plus_two = plus_two(8);

    // println!("The value of plus_two is {plus_two}");

    // let condition = true;
    // let number = if condition { 5 } else { 6 };

    // println!("The value of number is {number}");
    // loop_label();
    // loop_while()
    loop_arr_2()
}

fn loop_arr_2() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn loop_arr() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("the value is {}", a[index]);

        index += 1;
    }
}

fn loop_while() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!")
}

fn loop_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}")
}

fn loopper() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}")
}

fn another(x: i32, unit_label: char) {
    println!("Parameter: {x}");
}

fn stat() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is {y}")
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn plus_two(x: i32) -> i32 {
    let mut temp: i32 = x;
    temp = temp + 1;
    temp = temp + 1;
    temp
}
