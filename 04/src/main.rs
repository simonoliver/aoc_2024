use std::fs;
use std::ops::{Add, Sub};
use math2d;
use math2d::{Vector2i};

// Check in a given direction and a given position for a given pattern
fn check_pattern(pattern:&str,grid:&Vec<Vec<char>>, direction:Vector2i,position:Vector2i) -> bool
{
    let mut check_pos=position;
    for char in pattern.chars()
    {
        // Bounds check
        if check_pos.x<0 || check_pos.x>= grid[0].len() as i32 || check_pos.y<0 || check_pos.y>=grid.len() as i32
        {
            return false;
        }
        // Match char
        if grid[check_pos.x as usize][check_pos.y as usize]!=char
        {
            return false;
        }
        check_pos=check_pos.add(direction);
    }
    true
}

fn main()
{
    // All test directions
    let test_directions = vec![
        Vector2i::new(1,0),
        Vector2i::new(1,1),
        Vector2i::new(0,1),
        Vector2i::new(-1,1),
        Vector2i::new(-1,0),
        Vector2i::new(-1,-1),
        Vector2i::new(0,-1),
        Vector2i::new(1,-1),
    ];

    // The word to find
    let search_term="XMAS";
    let contents = fs::read_to_string("data/input")
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");
    // Use first line as ref
    let grid_width=lines.clone().into_iter().next().unwrap().len();
    let mut char_grid:Vec<Vec<char>>=Vec::new();

    // Build the 2D grid of characters
    for line in lines
    {
        if line.len()==grid_width
        {
            let char_array=line.chars().collect();
            char_grid.push(char_array);
        }
    }

    let grid_height=char_grid.len();
    println!("Grid size is {grid_width}x{grid_height}");

    let mut pass_count=0;
    for row in 0..grid_height
    {
        for column in 0..grid_width
        {
            for direction in &test_directions
            {
                if check_pattern(search_term, &char_grid, *direction, Vector2i::new(column as i32, row as i32))
                {
                    pass_count+=1;
                }
            }
        }
    }
    println!("Pass count is {pass_count}");

    // Part 2, check for specific pattern

    let pt2_search_term="MAS";
    // Valid directions
    let pt2_test_directions = vec![
        Vector2i::new(1,1),
        Vector2i::new(-1,1),
        Vector2i::new(-1,-1),
        Vector2i::new(1,-1),
    ];
    let mut patterns_found=0;
    for row in 0..grid_height
    {
        for column in 0..grid_width
        {
            let mut pos_pass_count=0;
            for direction in &pt2_test_directions
            {
                let mut start_position=Vector2i::new(column as i32, row as i32);
                // move one pos in the opposite direction to determine start pos
                start_position=start_position.sub(*direction);

                if check_pattern(pt2_search_term, &char_grid, *direction, start_position)
                {
                    pos_pass_count+=1;
                }
            }
            // If two diagonals found, pass
            if pos_pass_count==2
            {
                patterns_found += 1;
            }

        }
    }
    println!("PT2 - X-MAS patterns found {patterns_found}");
}
