use std::io;

fn main() {
    let mut prev1 = 0;
    let mut prev2 = 1;
    let mut input = String::new();
    let mut counter = 0;

    println!("Fibonacci till which number? ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let number: i64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("That wasn't a valid input!");
            return;
        }
    };

    while counter <= number {
        let fibo = prev1 + prev2;
        println!("{fibo}");

        prev1 = prev2;
        prev2 = fibo;

        counter += 1;
    }
}
