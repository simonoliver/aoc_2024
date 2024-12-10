use std::fs;

const DIRECTIONS:[(i32,i32);4]=[(1,0),(0,1),(-1,0),(0,-1)];

fn find_digit_positions(topographic_map : &Vec<Vec<u32>>, digit : u32) -> Vec<(i32,i32)>
{
    let mut found_positions: Vec<(i32,i32)>=Vec::new();
    for (row_index,row) in topographic_map.iter().enumerate() {
        for (column_index,value) in row.iter().enumerate() {
            if *value==digit {found_positions.push((column_index as i32,row_index as i32))}
        }
    }
    found_positions
}

fn find_trails_with_target_height(topographic_map : &Vec<Vec<u32>>, current_pos:(i32,i32), target_height:u32,allow_multiple_routes:bool) -> Vec<(i32,i32)>
{
    let mut found_trail_end_positions:Vec<(i32,i32)>=Vec::new(); // Track found trails
    let grid_width=topographic_map[0].len() as i32;
    let grid_height=topographic_map.len() as i32;
    let start_value=topographic_map[current_pos.1 as usize][current_pos.0 as usize];
    for direction in DIRECTIONS {
        let test_pos=(current_pos.0+direction.0,current_pos.1+direction.1);
        if test_pos.0>=0 && test_pos.0<grid_width && test_pos.1>=0 && test_pos.1<grid_height { // Bounds test
            let test_value=topographic_map[test_pos.1 as usize][test_pos.0 as usize];
            if test_value==start_value+1 {
                if test_value==target_height {
                    found_trail_end_positions.push(test_pos); // End of the trail as found target height
                } else {
                    let sub_trail_positions=find_trails_with_target_height(topographic_map,test_pos,target_height,allow_multiple_routes); // Recurse
                    for pos in sub_trail_positions {
                        if !found_trail_end_positions.contains(&pos) || allow_multiple_routes  {found_trail_end_positions.push(pos)};
                    }
                }
            }
        }
    }
    found_trail_end_positions
}

fn main() {
    let content=fs::read_to_string("data/input").expect("Expected to read the file");
    let lines=content.split("\n").filter(|line|line.len()>0);
    let mut topographic_map:Vec<Vec<u32>>=Vec::new();
    for line in lines {
        topographic_map.push(line.chars().into_iter().map(|height_char|height_char.to_digit(10).unwrap()).collect());
    }

    println!("Grid size {}x{}",topographic_map[0].len(),topographic_map.len());
    let trail_start_positions=find_digit_positions(&topographic_map,0);
    let mut found_trails_total=0;
    for trail_start_pos in trail_start_positions.clone() {
        
        found_trails_total+=find_trails_with_target_height(&topographic_map,trail_start_pos,9,false).len();
    }
    println!("Pt1 - Found trails total {}",found_trails_total);

    let mut pt2_found_trails_total=0;
    for trail_start_pos in trail_start_positions.clone() {
        pt2_found_trails_total+=find_trails_with_target_height(&topographic_map,trail_start_pos,9,true).len();
    }
    println!("Pt2 - Found trails total {}",pt2_found_trails_total);
    
}
