use std::collections::HashMap;

fn main() {
    let contents=std::fs::read_to_string("data/input").expect("Should be able to load");
    let lines=contents.split("\n");
    let grid_data:Vec<Vec<char>>=lines.filter(|line|!line.is_empty()).map(|line|line.chars().collect()).collect(); // Parse grid
    let grid_width=grid_data[0].len() as i32;
    let grid_height=grid_data.len() as i32;
    println!("Grid size {}x{}",grid_width,grid_height);

    // Build hashmap of char to vector of found locations
    let mut char_instance_hashmap:HashMap<char,Vec<(i32,i32)>>=HashMap::new();
    for (row_index,row) in grid_data.iter().enumerate() {
        for (column_index,grid_entry) in row.iter().enumerate()
        {
            if *grid_entry!='.' {
                char_instance_hashmap.entry(*grid_entry)   //https://users.rust-lang.org/t/how-do-i-do-insert-update-of-a-vec-inside-a-hashmap/17092/2
                    .or_default()
                    .push((column_index as i32, row_index as i32));
            }
        }
    }
    let mut antinode_location_hashmap:HashMap<(i32,i32),bool>=HashMap::new();
    for (_,point_list) in char_instance_hashmap.clone() {
        for point in &point_list {
            for point_compare in &point_list {
                if *point!=*point_compare {
                    let pos_diff=(point_compare.0-point.0,point_compare.1-point.1);
                    let antinode_point=(point.0-pos_diff.0,point.1-pos_diff.1); // Opposite
                    // Bounds check
                    if antinode_point.0>=0 && antinode_point.0<grid_width && antinode_point.1>=0 && antinode_point.1<grid_height {
                        antinode_location_hashmap.entry(antinode_point).or_insert(true);
                    }
                }
            }
        }
    }
    println!("Pt1 - Antinode location count {}",antinode_location_hashmap.keys().len());
    let mut antinode_location_hashmap:HashMap<(i32,i32),bool>=HashMap::new();
    for (_,point_list) in char_instance_hashmap.clone() {
        for point in &point_list {
            for point_compare in &point_list {
                if *point!=*point_compare {
                    let pos_diff=(point_compare.0-point.0,point_compare.1-point.1);
                    let mut offset_index=0;
                    let mut within_bounds=true;
                    while within_bounds {
                        let antinode_point=(point.0-pos_diff.0*offset_index,point.1-pos_diff.1*offset_index); // Opposite
                        if antinode_point.0>=0 && antinode_point.0<grid_width && antinode_point.1>=0 && antinode_point.1<grid_height { // Bounds check
                            antinode_location_hashmap.entry(antinode_point).or_insert(true);
                            offset_index+=1;
                        } else{
                            within_bounds=false;
                        }
                    }
                }
            }
        }
    }
    println!("Pt2 - Antinode location count {}",antinode_location_hashmap.keys().len());
}
