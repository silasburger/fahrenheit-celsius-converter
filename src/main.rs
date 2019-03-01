use std::io;

fn main() {

    println!("Enter your number here: ");

    let mut input_text = String::new();

    io::stdin().read_line(&mut input_text)
        .expect("Failed to read line");

    let trimmed = input_text.trim();

    let f_temp: f32 = trimmed.parse().unwrap();

    let c_temp: f32 = (f_temp - 32.00) * 5.0 / 9.0;

    println!("Here's your number {}", c_temp);
}





