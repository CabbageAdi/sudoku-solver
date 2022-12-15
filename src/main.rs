use std::io::{Read, Write};

const GRID_SIZE: usize = 9;
const GRID_DIVISION_SIZE: usize = 3;

fn main() {
    let mut grid: Vec<Vec<i32>> = vec![];
    input_grid(&mut grid);

    for _ in 0..GRID_SIZE * GRID_SIZE {
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                if grid[i][j] != 0 {
                    continue;
                }
                let r = check_row(&grid, i, j);
                let c = check_col(&grid, i, j);
                let b = check_box(&grid, i, j);

                let common: Vec<&i32> = r.iter().filter(|n| c.contains(n) && b.contains(n)).collect();
                if common.len() == 1 {
                    grid[i][j] = **common.first().unwrap();
                }
                else if common.len() == 0 {
                    panic!("No possibilities found at {} {} oops", i, j);
                }
            }
        }
    }

    display_grid(&grid);
}

fn check_row(grid: &Vec<Vec<i32>>, x: usize, y: usize) -> Vec<i32> {
    let row = &grid[x];
    (1..GRID_SIZE as i32 + 1).filter(|n| !row.contains(n)).collect()
}

fn check_col(grid: &Vec<Vec<i32>>, x: usize, y: usize) -> Vec<i32> {
    let col: Vec<i32> = grid.iter().map(|r| r[y]).collect();
    (1..GRID_SIZE as i32 + 1).filter(|n| !col.contains(n)).collect()
}

fn check_box(grid: &Vec<Vec<i32>>, x: usize, y: usize) -> Vec<i32> {
    let box_x = x / GRID_DIVISION_SIZE;
    let box_y = y / GRID_DIVISION_SIZE;

    let mut nums: Vec<i32> = vec![];
    for i in box_x * GRID_DIVISION_SIZE..(box_x + 1) * GRID_DIVISION_SIZE {
        for j in box_y * GRID_DIVISION_SIZE..(box_y + 1) * GRID_DIVISION_SIZE {
            if grid[i][j] != 0 {
                nums.push(grid[i][j]);
            }
        }
    }
    (1..GRID_SIZE as i32 + 1).filter(|n| !nums.contains(n)).collect()
}

fn input_grid(grid: &mut Vec<Vec<i32>>) {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut chars = buf.chars();
    for i in 0..GRID_SIZE * GRID_SIZE {
        if i % GRID_SIZE == 0 {
            grid.push(vec![]);
        }
        let mut c = chars.clone().nth(i).unwrap();
        if c == ' ' {
            c = '0';
        }
        grid.last_mut().unwrap().push(c.to_string().parse::<i32>().unwrap());
    }
    // for i in 0..GRID_SIZE {
    //     let mut row: Vec<i32> = vec![];
    //     for j in 0..GRID_SIZE {
    //         let mut buf = String::new();
    //         std::io::stdin().read_line(&mut buf).unwrap();
    //         buf = buf.trim().to_string();
    //         if buf == "" {
    //             buf = "0".to_string();
    //         }
    //         row.push(buf.parse::<i32>().unwrap());
    //     }
    //     grid.push(row);
    // }
}

fn display_grid(grid: &Vec<Vec<i32>>) {
    for row in grid {
        for n in row {
            print!("{}", n);
        }
        println!();
    }
}
