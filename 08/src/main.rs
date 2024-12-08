use std::collections::HashMap;
use math2d::Vector2i;

fn main() {

    let contents=std::fs::read_to_string("data/input").expect("Should be able to load");
    let lines=contents.split("\n");
    let mut grid_data:Vec<Vec<char>>=lines.filter(|line|line.len()>0).into_iter().map(|line|line.chars().collect()).collect(); // Parse grid
    let grid_width=grid_data[0].len() as i32;
    let grid_height=grid_data.len() as i32;
    println!("Grid size {}x{}",grid_width,grid_height);

    // Build hashmap of char to vector of found locations
    let mut char_instance_hashmap:HashMap<char,Vec<(i32,i32)>>=HashMap::new();
    for (row_index,row) in grid_data.iter().enumerate() {
        for (column_index,grid_entry) in row.iter().enumerate()
        {
            if *grid_entry!='.' {
                //https://users.rust-lang.org/t/how-do-i-do-insert-update-of-a-vec-inside-a-hashmap/17092/2
                println!("Adding {} at pos {},{}", *grid_entry, column_index, row_index);
                char_instance_hashmap.entry(*grid_entry)
                    .or_insert(Vec::new())
                    .push((column_index as i32, row_index as i32));
            }
        }
    }
    let mut antinode_location_hashmap:HashMap<(i32,i32),bool>=HashMap::new();
    for (char,point_list) in char_instance_hashmap {
        println!("Char {} entry count {}",char,point_list.len());
        for point in &point_list
        {
            for point_compare in &point_list
            {
                if *point!=*point_compare
                {
                    let pos_diff=(point_compare.0-point.0,point_compare.1-point.1);
                    let antinode_point=(point.0-pos_diff.0,point.1-pos_diff.1); // Opposite
                    // Bounds check
                    if antinode_point.0>=0
                        && antinode_point.0<grid_width
                        && antinode_point.1>=0
                        && antinode_point.1<grid_height
                        && !antinode_location_hashmap.contains_key(&antinode_point) {
                            antinode_location_hashmap.insert(antinode_point,true);
                        println!("Added antinode location at {},{} from point {},{}",antinode_point.0,antinode_point.1,point.0,point.1);
                    }
                }
            }
        }
    }
    println!("Antinode location count {}",antinode_location_hashmap.keys().len());
}
