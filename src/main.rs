use std::io;

fn main() {

    println!("Enter your number here: ");

    let mut input_text = String::new();

    io::stdin().read_line(&mut input_text)
        .expect("Failed to read line");

    let trimmed = input_text.trim();

    let f_temp: f32 = trimmed.parse().unwrap();

    let c_temp: f32 = (f_temp - 32.00) * 0.555;

    println!("Here's your number {}", c_temp);
}


// (32°F − 32) × 5/9 = 0°C

//   let mut input_text = String::new();
//     io::stdin()
//         .read_line(&mut input_text)
//         .expect("failed to read from stdin");

//     let trimmed = input_text.trim();
//     match trimmed.parse::<u32>() {
//         Ok(i) => println!("your integer input: {}", i),
//         Err(..) => println!("this was not an integer: {}", trimmed),
//     };