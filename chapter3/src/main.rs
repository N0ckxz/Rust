use std::io;

fn main() {
    f_to_c();

    // another_function();
    // control_flow();
}

fn f_to_c() {
    let mut selection = String::new();
    let mut input = String::new();
    let conversion: f32 = 0.0;
    // we ask the user for which conversion to use
    println!("SELECT CONVERSION: ");
    println!("1. C to F");
    println!("2. F to C");
    // store it in a variable
    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read line");

    let choice: i32 = match selection.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("That wasn't a valid selection!");
            return;
        }
    };

    if choice == 1 {
        // C to F conversion
        println!("Input temperature in C:");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let number: f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That wasn't a valid number!");
                return;
            }
        };
        let conversion = { number } / { 5.0 / 9.0 } + 32.0;
        println!("{number}C are = {conversion}F");
    } else if choice == 2 {
        // F to C conversion
        println!("Input temperature in F:");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let number: f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That wasn't a valid number!");
                return;
            }
        };
        let conversion = { number - 32.0 } * { 5.0 / 9.0 };
        println!("{number}F is = {conversion}C");
    } else {
        // Fallback
        println!("That wasn't a valid option!");
    }
}

// fn data_types() {
//     // declaration of a big integer, refer to documentation for sizes
//     // let x: i64 = 30000000000;
//     // println!("x is equal to {x} (and is an integer)");
//     //
//     // let _y = 4.2; // f64
//     //
//     // let _y: f32 = 17.5; // f32
//     // println!("y is equal to {_y} (and is a float)");
//     //
//     // // let subtraction = x - _y;
//     // // println!("subtraction: {subtraction}");
//     // // SO, we cant do that, but we can parse it to complete the operation I guess
//     //
//     // let _t = true;
//     //
//     // let _t: bool = false; // explicit type annotation
//     //
//     // let c = 'z';
//     // let z: char = 'ℤ'; // with explicit type annotation
//     // let heart_eyed_cat = '😻';
//     //
//     // // we can use emojis???
//     // let string_with_emojis = String::from("😀😀😀😀😀😀");
//     //
//     // println!("{heart_eyed_cat}");
//     // println!("{string_with_emojis}");
// }
// fn another_function() {
//     // println!("Another function.");
// }

// fn control_flow() {
//     // IF STATEMENTS
//     let number = 6;
//
//     if number < 5 {
//         println!("Condition was true");
//     } else {
//         println!("Condition was false");
//     }
//
//     // LOOPS
//     loop {
//         println!("Again!");
//     }
//
//     // CONDITIONAL LOOPS
//     let mut counter = 0;
//
//     let result = loop {
//         counter += 1;
//
//         println!("Counter: {counter}");
//
//         if counter == 15 {
//             break counter * 2;
//         }
//     };
//
//     println!("The result is {result}");
//
//     // WHILE LOOPS
//     let mut number = 3;
//
//     while number != 0 {
//         println!("{number}");
//
//         number -= 1;
//     }
//
//     println!("LIFTOFF!!!");
// }
