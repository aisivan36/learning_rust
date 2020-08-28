pub fn the_struct() {
    // let width1 = 30;
    // let height1 = 50;

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(width1, height1)
    // );

    // rect with tuples
    // rect_test();
    // Refactoring with Structs: Adding More Meaning
    // struct_test();
    //Adding Useful Functionality with Derived Traits
    test_trait();
}

// Refactoring with Structs: Adding More Meaning
// fn struct_test() {
//     let rect3 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         test_area(&rect3)
//     );
// }

// fn test_area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn rect_test() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area1(rect1)
//     );
// }

// fn area1(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// fn area(width: u32, height: u32) -> u32 {
//     width + height
// }

//Adding Useful Functionality with Derived Traits
fn test_trait() {
    let rect1 = Rectangle2 {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:#?}", rect1.area());
}

impl Rectangle2 {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// with debug mode and put :? or :#? into {}
// and use this #[derive(Debug)] at the above of struct thats all
#[derive(Debug)]
struct Rectangle2 {
    width: u32,
    height: u32,
}
