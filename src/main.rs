mod ownership;
mod struct_test;

fn main() {
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    println!(
        "addition: {}, subtraction: {}, multiplication: {}, division: {}, remainder: {} ",
        sum, difference, product, quotient, remainder
    );

    //Touple

    // let x: (i32, f64, u8) = (500, 6.4, 1);

    // let five_hundred = x.0;

    // let six_point_four = x.1;

    // let one = x.2;

    // println!(
    //     "The value of y is: {}, {}, {}",
    //     five_hundred, six_point_four, one,
    // );

    // Arrays

    // let a = [1, 2, 3, 4, 5];

    // let index = 10;

    // let element = a[index];

    // println!("The arays are: {}", element);

    // test_fuction(7, 9);

    // ownership imported
    ownership::testownership();
    // struct Imported
    struct_test::the_struct();
}

// fn five() -> i32 {
//     5
// }

// fn test_fuction(x: i32, z: i32) {
//     println!("the value of x is: {}, and the z is: {}", x, z);

//     let y = {
//         let x = 3;
//         x + 1
//     };
//     println!("the value of y is: {} ", y);

//     let x = five();
//     let y = x;
//     let z = y + 5;

//     println!("the value of x is: {}", z);
//     if_statement();
// }

// fn if_statement() {
//     let number = 9;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3 or 2");
//     }

// If number is not equal to

// let number = 3;
// if number != 0 {
//     println!("Number was something other than zero");
// }

// using if in a let statement

// let condition = true;
// let number = if condition { 5 } else { 6 };
// println!("The value of number is: {}", number);

// Repeating code with loop
// loop {
// println!("again!");
// }
// let mut counter = 0;
// let result = loop {
//     counter += 1;

//     if counter == 11 {
//         break counter * 2;
//     }
// };
// println!("The result is {}", result);

// Conditional loops with while
// let mut number = 3;
// while number != 0 {
//     println!("{}!", number);
//     number -= 1;
// }
// println!("LIFTOFF!!!")

// Looping While
// let a = [10, 20, 30, 40, 50];
// let mut index = 0;

// while index < 5 {
//     println!("the value is: {}", a[index]);

//     index += 1;
// }
// an alternative use loop
// Recommended to use this case rather than while
//     let a = [10, 20, 30, 40, 50];

//     for element in a.iter() {
//         println!("the value of while is: {}", element);
//     }
//     // or this one
//     for number in (0..5).rev() {
//         println!("{}", number);
//     }
//     println!("It was succed");
// }
