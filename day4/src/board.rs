

pub struct Board {
    height : u8,
    width : u8,
    grid : Vec<u16>,
    drawn_grid : Vec<bool>
}

impl Board {

    pub fn new(grid_data : &Vec<u16> ) -> Board {
        let h : u8 = 5;
        let w : u8= 5;
        let total_size : usize = (h * w).into();
        Board {
            height : h,
            width : w,
            grid : grid_data.clone(),
            drawn_grid : vec![false; total_size]
        }
    }

    pub fn draw_number(&mut self, number : u16)  {
        for i in 0..self.height {
            for j in 0..self.width {
                let index : usize = (i * self.width + j).into();
                if self.grid[index] == number {
                    self.drawn_grid[index] = true;
                    return;
                }
            }
        }
    }

    pub fn has_won(&self) -> bool {
        // Checks rows
        for i in 0..self.height {
            let mut won  = true;

            for j in 0..self.width {
                let index : usize = (i * self.width + j).into();
                let index2 = index as u32;
                won = won & self.drawn_grid[index];
            }
            if won {
                return won;
            }
        }

        // Checks columns
        for j in 0..self.width {
            let mut won  = true;

            for i in 0..self.height {
                let index : usize = (i * self.width + j).into();
                let index2 = index as u32;
                won = won & self.drawn_grid[index];
            }
            if won {
                return won;
            }
        }
        false
    }

    pub fn remaining_numbers(&self) -> Vec<u16> {
        let mut result : Vec<u16> = vec!();
        for j in 0..self.width {
            for i in 0..self.height {
                let index : usize = (i * self.width + j).into();
                if !self.drawn_grid[index] {
                    result.push(self.grid[index]);
                }
            }
        }
        result
    }

    
    
}

