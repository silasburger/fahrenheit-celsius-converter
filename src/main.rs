use std::{io, process};

enum Temp {
    C(f64),
    F(f64),
}

fn convert_temp(temp: &Temp) -> f64 {
    match temp {
        &Temp::F(degrees) => (degrees - 32.0) / 1.8,
        &Temp::C(degrees) => (degrees * 1.8) + 32.0,
    }
}

fn print_temp(temp: &Temp) {
    match temp {
        &Temp::F(degrees) => println!("{}F = {}C", degrees, convert_temp(temp)),
        &Temp::C(degrees) => println!("{}C = {}F", degrees, convert_temp(temp)),
    };
}

fn main() {
    println!("Press q to quit at any time");
    loop {
        let mut input_text = String::new();
        println!("Enter a number followed by a C or F (Format: 32F): ");
        io::stdin()
            .read_line(&mut input_text)
            .expect("Failed to read line");

        let trimmed = input_text.trim();

        if trimmed == "q" {
            process::exit(1);
        };

        let (temp, scale) = trimmed.split_at(trimmed.len() - 1);
        let temp: f64 = match temp.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let temp = match scale {
            "C" => Temp::C(temp),
            "F" => Temp::F(temp),
            _ => continue,
        };

        print_temp(&temp);
    }
}
