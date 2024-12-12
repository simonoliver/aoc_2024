
use std::fs;

const NEIGHBOUR_DIRECTIONS:[(i32,i32);4]=[(1,0),(0,1),(-1,0),(0,-1)];

fn add_region_neighbours_at_location(garden_data:&Vec<Vec<char>>, region_location:(i32,i32),found_locations:&mut Vec<(i32,i32,i32)>,processed_locations_list:&mut Vec<bool>) {
    let mut found_neighbours=0;
    let current_index=region_location.0+region_location.1*(garden_data[0].len() as i32);
    processed_locations_list[current_index as usize]=true; // Mark as processed

    for direction in NEIGHBOUR_DIRECTIONS {
        let test_location=(region_location.0+direction.0,region_location.1+direction.1);
        if test_location.0>=0 && test_location.0<(garden_data[0].len() as i32) && test_location.1>=0 && test_location.1<(garden_data.len() as i32) { // Bounds check
            let location_index=test_location.0+test_location.1*(garden_data[0].len() as i32);
            if garden_data[test_location.1 as usize][test_location.0 as usize]==garden_data[region_location.1 as usize][region_location.0 as usize] { // Char match
                found_neighbours+=1;
                if !processed_locations_list[location_index as usize] { // If not processed, add
                    add_region_neighbours_at_location(garden_data,test_location,found_locations,processed_locations_list);  // Recuse for the new location
                }
            }
        }
    }
    found_locations.push((region_location.0,region_location.1,found_neighbours)); // Add this location with neighbour count
}

fn find_regions(garden_data:&Vec<Vec<char>>) -> Vec<Vec<(i32,i32,i32)>> {
    let mut region_list:Vec<Vec<(i32,i32,i32)>>=Vec::new();
    let (grid_width,grid_height) = (garden_data[0].len(),garden_data.len());
    let total_locations=grid_width*grid_height;
    let mut processed_locations_list:Vec<bool>=vec![false;total_locations];

    for (rowindex,row) in garden_data.iter().enumerate() {
        for (column_index,_) in row.iter().enumerate() {
            let process_index=rowindex*grid_width+column_index;
            if !processed_locations_list[process_index] {
                // New region
                let mut new_region_list:Vec<(i32,i32,i32)>=Vec::new();
                add_region_neighbours_at_location(garden_data,(column_index as i32,rowindex as i32),&mut new_region_list,&mut processed_locations_list);
                region_list.push(new_region_list);
            }
        }
    }
    region_list
}

fn main() {
    let content = fs::read_to_string("data/test_input").expect("Expected to read the file");

    let lines=content.split("\n").filter(|line|line.len()>0);
    let garden_data:Vec<Vec<char>>=lines.map(|line|{
        line.chars().collect()
    }).collect();

    for line in &garden_data {
        println!("Line {:?}",line);
    }
    let found_regions=find_regions(&garden_data);
    for region in found_regions {
        println!("Region size {}",region.len());
    }

}


