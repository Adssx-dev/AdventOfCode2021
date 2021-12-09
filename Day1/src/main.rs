use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    part_one();
    part_two();
}

fn part_one(){
    let mut previous : u32 = std::u32::MAX;
    let mut total : u32 =0;
    
    if let Ok(lines) = read_lines("./input.txt") {


        for line in lines {
            if let Ok(value_str) = line {
                let new_val = value_str.parse::<u32>().unwrap();
                if new_val > previous {
                    total += 1;
                }
                previous = new_val;
            }
        }
    }
    println!("Result : {}", total);
}

fn part_two(){
    let max : u32 = std::u32::MAX / 4 - 1;
    let mut total : u32 =0;
    let mut previous :[u32; 4] = [max, max, max, max];
    
    if let Ok(lines) = read_lines("./input.txt") {

        for line in lines {
            if let Ok(value_str) = line {

                for i in [0, 1, 2] {
                    previous[i] = previous[i + 1];
                }

                previous[3] = value_str.parse::<u32>().unwrap();
                
                let a = previous[1..].iter().sum::<u32>();
                let b = previous[..3].iter().sum();

                if a > b{
                    total += 1;
                }
            }
        }
    }
    println!("Result : {}", total);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// use std::fs::File;
// use std::io::{self, prelude::*, BufReader};

// fn main() {
//     let first = true;
//     let mut previous : u32 = 0;
//     let mut total : u32 =0;
    
//     if let Ok(lines) = read_lines("./input.txt") {


//         for line in lines {
//             if let Ok(value_str) = line {
//                 let new_val = value_str.parse::<i32>().unwrap();
//                 if(new_val > previous) {
//                     total += 1;
//                 }
//                 previous = new_val;
//             }
//         }
//     }
//     println!("Result : {}", total);
// }
