pub fn testownership() {
    // let mut s = String::from("TestRUst");

    // s.push_str(", for learning");

    // println!("{}", s);

    let s1 = String::from("works, ownership function ");
    let s2 = s1.clone();

    println!("it {}, s1 = {}", s2, s1);

    // Take ownership
    let mut s = String::from("Hello, ");
    s.push_str("You are the");
    s.push_str(" Best Person");

    // takes_ownership(s);

    // let x = 5;

    // makes_copy(x);

    // Return values and scope
    // let s1 = gives_ownership();
    // let s2 = String::from("You got ");
    // let s3 = takes_and_gives_back(s2);
    // println!("{}, {}", s3, s1);
    //Second
    let mut s1 = String::from("Bounjur");
    change(&mut s1);

    main_test();
}

// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }

// fn makes_copy(some_interger: i32) {
//     println!("{}", some_interger);
// }

// Return values and scope

// fn gives_ownership() -> String {
//     let with_string = String::from("Bounjur");
//     with_string
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }

fn change(some_string: &mut String) {
    some_string.push_str(", Ivan");
    println!("{}", some_string);

    // let mut s = String::from("Hi ivan");
    // let r1 = &s;
    // let r2 = &s;
    // println!("{} and {}", r1, r2);

    // let r3 = &mut s;
    // println!("{}", r3);

    // Dangling References
    // let mut s = String::from("Hello Ivan");
    // let word = dangle(&s);
    // s.clear();

    // test();

    // let mut s = String::from("whati kso");
    // let word = test_error(&s);
    // s.clear();

    // println!("the first word is: {}", word);
}

// fn dangle() -> String {
//     // THe Slice Type
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
//     // println!("{}", bytes);
// }

// #![allow(unused_variables)]
// fn test() {
//     let s = String::from("half works");

//     let len = s.len();

//     let _slice = &s[0..len];
//     let slice = &s[..];

//     println!("{}", slice)
// }

// second test error
// fn test_error(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     &s[..]
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            println!("{:?}", i);
            let a = &s[0..i]; //nek iki  referensi kata s mulai indels 0 sampek 5 dan nek 4 iku wes metu soale 5 ra diitung
            println!("{:?}", a);
            return a;
        }
    }

    &s[..]
}

fn main_test() {
    // let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    // let word = first_word(&my_string[..]);

    let my_string_literal = "hellos worlds";

    // first_word works on slices of string literals
    // let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    println!("{}", word);
    print!("{}", my_string_literal);

    let s = String::from(", whatisup");

    let slice = &s[0..];
    let slice2 = &s[1..];
    print!("{}, {}", slice, slice2);

    //Slices by ivan
    let his_string = String::from("Bounjur worlds");

    let took = my_code(&his_string);
    let took2 = his_string.len();

    println!("{},the character is: {}", took, took2);
    println!("{}", his_string);
}

fn my_code(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            println!("the end of :{}", i);
            let var = &s[0..i];
            println!("{}", var);
            let sa = var.len();
            // let sa3 = i.len();

            // println!("{}", sa3);

            println!("{}", sa);
        }
    }

    &s[..]
}
