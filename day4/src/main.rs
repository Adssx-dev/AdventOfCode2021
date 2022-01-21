use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::string;

use board::Board;
mod board;

fn main() {
    let mut result = part_one();
    println!("{}", result);

    result = part_two();
    println!("{}", result);
}

fn part_two() -> u32 {

    let (mut grids, draws) = init_grids_and_draws();

    for draw in draws.clone() {
        for grid  in &mut grids {
            grid.draw_number(draw);
        }
        let mut won_grids = 0;
        let mut a_not_won_grid = 0;

        let mut index : usize = 0;
        for grid  in &mut grids {
            if grid.has_won() {
                won_grids += 1;
            }
            else {
                a_not_won_grid = index;
            }
            index += 1;
        }
        if won_grids == grids.len() - 1 {
            let last_grid = &mut grids[a_not_won_grid];
            for second_draw in draws.clone() {
                last_grid.draw_number(second_draw);
                if last_grid.has_won() {
                    let sum :u16 = last_grid.remaining_numbers().iter().sum();
                    return (sum as u32) * (second_draw as u32);
                }
            }
        }
    }
    0
}

fn part_one() -> u32 {

    let (mut grids, draws) = init_grids_and_draws();

    for draw in draws {
        for grid  in &mut grids {
            grid.draw_number(draw);
            if grid.has_won() {
                let sum : u16 = grid.remaining_numbers().iter().sum();
                return (sum as u32) * (draw as u32);
            }
        }
    }
    0
}

fn init_grids_and_draws () -> (Vec<Board>, Vec<u16>) {
    let mut grids : Vec<Board> = vec!();
    let mut draws_u16 : Vec<u16> = vec!();

    if let Ok(mut lines) = read_lines("./input.txt") {
        let draws  = lines.next().unwrap().unwrap();
        let draws_str : Vec<&str> = draws.split(",").collect();
        
        for i in 0..draws_str.len() {
            draws_u16.push(draws_str[i].parse::<u16>().unwrap());
        }

        let mut grid_values : Vec<u16> = vec!();

        while let Some(line) = lines.next() {
            let l = line.unwrap();
            if l.eq("") && grid_values.len() > 0 {
                grids.push(Board::new(&grid_values));
                grid_values = vec!();
            }
            else {
                let mut grid_line : Vec<u16> = l.split(" ").filter(|l| l.len() > 0).map(|l| l.parse::<u16>().unwrap()).collect();
                grid_values.append(&mut grid_line);
            }
        }
    }
    (grids, draws_u16)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

