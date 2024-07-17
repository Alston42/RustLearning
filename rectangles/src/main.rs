// fn main() {
//     let width = 30;
//     let height = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width, height)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }


// fn main() {
//     let rect: (u32, u32) = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect)
//     );
// }

// fn area(size: (u32, u32)) -> u32 {
//     size.0 * size.1
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect),
    );

    println!("rect is {rect:?}")
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}