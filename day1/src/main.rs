use std::fs;
fn main() {
    let input = load_input();

    let mut list_of_int: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut first_integer = String::new();
        let mut last_integer = String::new();

        for character in line.chars() {
            if character.is_digit(10) {
                if first_integer.is_empty() {
                    first_integer.push(character);
                }
                last_integer = character.to_string();
            }
        }

        let combined_integer = format!("{}{}", first_integer, last_integer);

        list_of_int.push(combined_integer.parse::<i32>().unwrap());

        println!("First integer: {}, Last integer: {}, Combined: {}", first_integer, last_integer, combined_integer);
    }

    let mut sum_of_integers = 0;
    for integer in &list_of_int {
        sum_of_integers += integer;
    }

    println!("Sum of all integers: {}", sum_of_integers);
}

fn load_input() -> String {
    let input = fs::read_to_string("puzzle-input.txt")
        .expect("Something went wrong reading the file");
    return input;
}
