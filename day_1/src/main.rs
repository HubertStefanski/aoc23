use std::fs;

fn get_first_num(input_string: &str) -> i32 {
    input_string.chars().find(|c| c.is_numeric()).map_or(0, |c| c as i32 - '0' as i32)
}

fn main() {
    let file_path = "input.txt";

    let sum: i32 = fs::read_to_string(file_path)
        .expect("Unable to read file")
        .lines()
        .map(|line| {
            let reversed_line: String = line.chars().rev().collect();
            let sum_string = format!("{}{}", get_first_num(line), get_first_num(&reversed_line));
            sum_string.parse::<i32>().unwrap_or(0)
        })
        .sum();

    println!("{}", sum);
}
