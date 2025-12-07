use std::{env, fs};

fn find_neighbours(grid: &mut Vec<Vec<char>>, dir: &[(i64, i64);9]) -> u64 {
    let mut tp_count = 0;
    
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            
            if grid[x][y] != '@' { continue; }
            
            let mut count: u64 = 0;
            
            for (dir_x, dir_y) in dir {

                let (new_x, new_y) = ((x as i64 + dir_x) as usize, (y as i64 + dir_y) as usize); //theres gotta be a better way to do this :sob:
                
                if new_x < grid.len() && new_y < grid[0].len() {
                    let spot = grid[new_x][new_y];
                    if spot == '@' || spot == 'x'{
                        count += 1;
                    }
                }
            }
            if count < 5 {
                tp_count += 1;
                grid[x][y] ='x';
            }
        }
    }
    tp_count
}

pub fn get_toilet_paper() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = &args[2];
    
    let data = fs::read_to_string(file_name).expect("Unable to read file.");
    let mut grid: Vec<Vec<char>> = data.lines()
                                    .map(|line| line.chars().collect())
                                    .collect();


    let dir: [(i64, i64);9] = [
                                (-1, -1),
                                (0, -1),
                                (1, -1),
                                (-1, 0),
                                (0, 0),
                                (1, 0),
                                (-1, 1),
                                (0, 1),
                                (1, 1)
                            ];

    let tp_count_p1: u64 = find_neighbours(&mut grid, &dir);
    let mut tp_count_p2 = 0;

    loop {
        for x in 0..grid.len() {
            for y in 0..grid[0].len() {
                if grid[x][y] == 'x' { grid[x][y] = '.'; }
            }
        }

        let next_count = find_neighbours(&mut grid, &dir);
        tp_count_p2 += next_count;
        
        if next_count == 0 { break };
    }

    tp_count_p2 += tp_count_p1;

    println!("{tp_count_p1}");
    println!("{tp_count_p2}");
}