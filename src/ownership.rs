pub fn testownership() {
    // let mut s = String::from("TestRUst");

    // s.push_str(", for learning");

    // println!("{}", s);

    let s1 = String::from("works");
    let s2 = s1.clone();

    println!("it {}, s1 = {}", s2, s1);

    // Take ownership
    let mut s = String::from("Hello, ");
    s.push_str("You are the");
    s.push_str(" Best Person");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);

    // Return values and scope
    // let s1 = gives_ownership();
    // let s2 = String::from("You got ");
    // let s3 = takes_and_gives_back(s2);
    // println!("{}, {}", s3, s1);
    //Second
    let s1 = String::from("Bounjur");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_interger: i32) {
    println!("{}", some_interger);
}

// Return values and scope

// fn gives_ownership() -> String {
//     let with_string = String::from("Bounjur");
//     with_string
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }

fn calculate_length(ds: &String) -> usize {
    ds.len()
}
