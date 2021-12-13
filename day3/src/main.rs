use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const line_length: usize = 12;

fn main() {
    part_one();
    part_two();
}

fn part_two() {
    let mut array: Vec<Vec<char>> = Vec::new();

    // Fill the table
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(value_str) = line {
                array.push(value_str.chars().collect());
            }
        }
    }

    let mut array2 = array.clone();

    for i in 0..(line_length) {
        let sum : usize = array.iter().map(|v| if v[i] == '1' {1} else {0}).sum();
        let count = array.iter().count();
        
        let target:char;
        if count - sum == sum {
            target = '1';
        }
        else if count - sum > sum {
            target = '0';
        }
        else {
            target = '1';
        }
        array = array.iter().filter(|v| v[i] == target).map(|v| v.clone()).collect();
        if array.iter().count() == 1 {
            break;
        }
    };
    let oxygen_rating =  &array[0];

    println!("{:?}", array);
    
    for i in 0..(line_length) {
        let sum : usize = array2.iter().map(|v| if v[i] == '1' {1} else {0}).sum();
        let count = array2.iter().count();
        
        let target:char;
        if count - sum == sum {
            target = '0';
        }
        else if count - sum > sum {
            target = '1';
        }
        else {
            target = '0';
        }
        array2 = array2.iter().filter(|v| v[i] == target).map(|v| v.clone()).collect();
        if array2.iter().count() == 1 {
            break;
        }
    };
    let co2_rating =  &array2[0];
    println!("{:?}", array2);

    println!("Oxygen : {} \n CO2 : {} \n Total : {}", bin_char_vec_to_integer(oxygen_rating), bin_char_vec_to_integer(co2_rating), bin_char_vec_to_integer(oxygen_rating) * bin_char_vec_to_integer(co2_rating))
}

fn part_one() {
    let mut count: u32 = 0;
    let mut array: [u32; line_length] = [0; 12];

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(value_str) = line {
                count += 1;
                for i in 0..line_length {
                    if value_str.chars().nth(i).unwrap() == '1' {
                        array[i] += 1;
                    }
                }
            }
        }
    }
    let mut gamma: [bool; line_length] = [false; 12];
    let mut epsilon: [bool; line_length] = [false; 12];

    for i in 0..line_length {
        gamma[i] = array[i] > count / 2;
        epsilon[i] = !gamma[i];
    }

    let gamma_int = bin_string_to_integer(gamma);
    let epsilon_int = bin_string_to_integer(epsilon);

    println!("Gamma : {}", gamma_int);
    println!("Epsilon : {}", epsilon_int);
    println!("Result : {}", gamma_int * epsilon_int);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn bin_char_vec_to_integer(array : &Vec<char>) -> u32 {
    let mut result: u32 = 0;
    let mut current_power: u32 = 1;
    for i in (0..line_length).rev() {
        if array[i] == '1' {
            result += current_power;
        }
        current_power *= 2;
    }
    result
}

fn bin_string_to_integer(array: [bool; line_length]) -> u32 {
    let mut result: u32 = 0;
    let mut current_power: u32 = 1;
    for i in (0..line_length).rev() {
        if array[i] {
            result += current_power;
        }
        current_power *= 2;
    }
    result
}
