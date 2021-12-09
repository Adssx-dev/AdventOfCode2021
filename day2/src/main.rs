use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    part_one();
    part_two();
}

fn part_two() {
    let mut x_pos :i32 = 0;
    let mut y_pos :i32 = 0;
    let mut aim = 0;
    if let Ok(lines) = read_lines("./input.txt") {

        for line in lines {
            if let Ok(value_str) = line {
                let mut splitted = value_str.split(" ");
                let command = splitted.next().unwrap();
                let value : i32= splitted.next().unwrap().parse::<i32>().unwrap();
                match command {
                    "forward" => {
                        x_pos += value;
                        y_pos += aim * value;
                    },
                    "down" => aim += value,
                    "up" => aim -= value,
                    _ => panic!("Invalid command")
                }
            }
        }
    }
    println!("{}", x_pos * y_pos);
}

fn part_one() {
    let mut x_pos :i32 = 0;
    let mut y_pos :i32 = 0;
    if let Ok(lines) = read_lines("./input.txt") {

        for line in lines {
            if let Ok(value_str) = line {
                let mut splitted = value_str.split(" ");
                let command = splitted.next().unwrap();
                let value : i32= splitted.next().unwrap().parse::<i32>().unwrap();
                match command {
                    "forward" => x_pos += value,
                    "down" => y_pos += value,
                    "up" => y_pos -= value,
                    _ => panic!("Invalid command")
                }
            }
        }
    }
    println!("{}", x_pos * y_pos);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
