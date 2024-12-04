use std::fs;
use std::num::ParseIntError;
use std::ops::Add;
use math2d;
use math2d::{vector2i, Vector2i};

// Check in a given direction and a given position for a given pattern
fn check_pattern(pattern:&str,grid:&Vec<Vec<char>>, direction:Vector2i,position:Vector2i) -> bool
{
    let mut check_pos=position;
    let x=check_pos.x;
    let y=check_pos.y;
    println!("Checking pos {x},{y} string {pattern}");
    for char in pattern.chars()
    {
        if check_pos.x<0 || check_pos.x>= grid[0].len() as i32 || check_pos.y<0 || check_pos.y>=grid.len() as i32
        {
            return false;
        }
        if grid[check_pos.x as usize][check_pos.y as usize]!=char
        {

            // Check match
            return false;
        }
        check_pos.add(direction);
    }
    true
}

fn main()
{

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

    let search_term="XMAS";
    let contents = fs::read_to_string("data/test_input")
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");
    // Use first line as ref
    let grid_width=lines.clone().into_iter().next().unwrap().len();
    let mut grid_height=0;
    let mut char_line_vec:Vec<Vec<char>>=Vec::new();

    for line in lines
    {
        if (line.len()==grid_width)
        {
            let char_array=line.chars().collect();
            char_line_vec.push(char_array);
        }
    }

    let grid_height=char_line_vec.len();
    println!("Grid size is {grid_width}x{grid_height}");

    let mut pass_count=0;
    for row in 0..grid_height
    {
        for column in 0..grid_width
        {
            for direction in &test_directions
            {
                if check_pattern(search_term, &char_line_vec, *direction, Vector2i::new(column as i32, row as i32))
                {
                    pass_count+=1;
                }
            }
        }
    }
    println!("Pass count is {pass_count}");
    /*
    // From https://stackoverflow.com/questions/13212212/creating-two-dimensional-arrays-in-rust
    let mut grid_raw = vec![0; grid_width * grid_height];

    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(grid_width).collect();

    // Final 2d array `&mut [&mut [_]]`
    let grid = grid_base.as_mut_slice();

    // Accessing data
    grid[0][0] = 4;

    for line in lines
    {

    }
    */
}
