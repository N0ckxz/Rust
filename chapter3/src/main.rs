use std::io;

fn main() {
    // declaration of a big integer, refer to documentation for sizes
    let x: i64 = 30000000000;
    println!("x is equal to {x} (and is an integer)");

    let _y = 4.2; // f64

    let _y: f32 = 17.5; // f32
    println!("y is equal to {_y} (and is a float)");

    // let subtraction = x - _y;
    // println!("subtraction: {subtraction}");
    // SO, we cant do that, but we can parse it to complete the operation I guess

    let _t = true;

    let _t: bool = false; // explicit type annotation

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // we can use emojis???
    let string_with_emojis = String::from("😀😀😀😀😀😀");

    println!("{heart_eyed_cat}");
    println!("{string_with_emojis}");
}
