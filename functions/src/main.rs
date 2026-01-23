use std::{char, io};

fn main() {
    println!("Enter C to convert to Celcius or F to convert to Farenheit");

    let mut cel_or_far = String::new();

    io::stdin()
        .read_line(&mut cel_or_far)
        .expect("Failed to read direction");

    let cel_or_far: char = match cel_or_far.trim().parse() {
        Ok(char) => char,
        Err(_) => return,
    };

    println!("now enter temperature to convert");

    let mut temp = String::new();
    
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp: i32 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let new_temp: i32 = convert_temp(temp,cel_or_far);

    println!("old temp: {temp}, new temp {new_temp}{cel_or_far}")
}

fn convert_temp(x: i32,y: char) -> i32{
    if y=='C' {
        (x-32) * 5/9
    }
    else {
        (x * 9/5) + 32
    }
}