
use std::fs;

const NEIGHBOUR_DIRECTIONS:[(i32,i32);4]=[(1,0),(0,1),(-1,0),(0,-1)];
// Fot all diag neighbours check. Order is → ↘ ↓ ↙ ← ↖ ↑ ↗
const ALL_NEIGHBOUR_DIRECTIONS:[(i32,i32);8]=[(1,0),(1,1),(0,1),(-1,1),(-1,0),(-1,-1),(0,-1),(1,-1)];

fn add_region_neighbours_at_location(garden_data:&Vec<Vec<char>>, region_location:(i32,i32),found_locations:&mut Vec<(i32,i32,i32,u8)>,processed_locations_list:&mut Vec<bool>) {
    let mut found_neighbours=0;

    let current_index=region_location.0+region_location.1*(garden_data[0].len() as i32);
    processed_locations_list[current_index as usize]=true; // Mark as processed

    for (direction_index,direction) in NEIGHBOUR_DIRECTIONS.iter().enumerate() {
        let test_location=(region_location.0+direction.0,region_location.1+direction.1);
        if test_location.0>=0 && test_location.0<(garden_data[0].len() as i32) && test_location.1>=0 && test_location.1<(garden_data.len() as i32) { // Bounds check
            let location_index=test_location.0+test_location.1*(garden_data[0].len() as i32);
            if garden_data[test_location.1 as usize][test_location.0 as usize]==garden_data[region_location.1 as usize][region_location.0 as usize] { // Char match
                found_neighbours+=1;
                //neighbors_bits|=1<<direction_index; // Add neighbor bit
                if !processed_locations_list[location_index as usize] { // If not processed, add
                    add_region_neighbours_at_location(garden_data,test_location,found_locations,processed_locations_list);  // Recuse for the new location
                }
            }
        }
    }
    let mut neighbors_bits:u8=0;
    for (direction_index,direction) in ALL_NEIGHBOUR_DIRECTIONS.iter().enumerate() {
        let test_location=(region_location.0+direction.0,region_location.1+direction.1);
        if test_location.0>=0 && test_location.0<(garden_data[0].len() as i32) && test_location.1>=0 && test_location.1<(garden_data.len() as i32) { // Bounds check
            let location_index=test_location.0+test_location.1*(garden_data[0].len() as i32);
            if garden_data[test_location.1 as usize][test_location.0 as usize]==garden_data[region_location.1 as usize][region_location.0 as usize] { // Char match
                neighbors_bits|=1<<(7-direction_index); // Add neighbor bit
            }
        }
    }
    println!("Neighbors bits {neighbors_bits}");
    found_locations.push((region_location.0,region_location.1,found_neighbours,neighbors_bits)); // Add this location with neighbour count
}

fn find_regions(garden_data:&Vec<Vec<char>>) -> Vec<Vec<(i32,i32,i32,u8)>> {
    let mut region_list:Vec<Vec<(i32,i32,i32,u8)>>=Vec::new();
    let (grid_width,grid_height) = (garden_data[0].len(),garden_data.len());
    let total_locations=grid_width*grid_height;
    let mut processed_locations_list:Vec<bool>=vec![false;total_locations];

    for (rowindex,row) in garden_data.iter().enumerate() {
        for (column_index,_) in row.iter().enumerate() {
            let process_index=rowindex*grid_width+column_index;
            if !processed_locations_list[process_index] {
                // New region
                let mut new_region_list:Vec<(i32,i32,i32,u8)>=Vec::new();
                add_region_neighbours_at_location(garden_data,(column_index as i32,rowindex as i32),&mut new_region_list,&mut processed_locations_list);
                region_list.push(new_region_list);
            }
        }
    }
    region_list
}

fn side_count (side_bits:u8) -> u8
{
    // TODO Need to factor in diagonals, as will create a valid side if diagonal present but still
    // an entry to the left

    //[(1,0),(0,1),(-1,0),(0,-1)]; Right, down, left, top . RDLT
    let mut side_count:u8=0;
    //                    →↘↓↙←↖↑↗      →↘↓↙←↖↑↗
    if side_bits &      0b00001010 == 0b00000000 {side_count+=1} // Nothing above and left
    else if side_bits & 0b00000110 == 0b00000100 {side_count+=1}; // Nothing above but have above left
    if side_bits &      0b10000010 == 0b00000000 {side_count+=1} // Nothing right and top
    else if side_bits & 0b10000001 == 0b00000001 {side_count+=1}; // Nothing right but have top right
    if side_bits &      0b10100000 == 0b00000000 {side_count+=1} // Nothing down and right
    else if side_bits & 0b01100000 == 0b01000000 {side_count+=1}; // Nothing down but have down right
    if side_bits &      0b00101000 == 0b00000000 {side_count+=1} // Nothing left and down
    else if side_bits & 0b00011000 == 0b00010000 {side_count+=1}; // Nothing left but have down left
    side_count
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
    let mut pt1_total=0;
    let mut pt2_total=0;

    for region in &found_regions {
        let region_size=region.len();
        let region_borders=region.iter().fold(0,|acc,region_entry|acc+(4-region_entry.2));
        let side_count=region.iter().fold(0,|acc,region_entry|acc+side_count(region_entry.3));
        let first_entry=region[0];
        let first_char=garden_data[first_entry.1 as usize][first_entry.0 as usize];
        println!("Region {} size {} borders {} side_count {}",first_char,region_size,region_borders,side_count);
        pt1_total+=region_size as i32*region_borders;
        pt2_total+=region_size as i32*(side_count as i32);
    }
    let region1=&found_regions[0];
    for location in region1 {
        let sides=side_count(location.3);
        println!("Location {},{} {:b} side count {}",location.0,location.1,location.3,sides);
    }
    println!("Pt1 Total {pt1_total}");
    println!("Pt2 Total {pt2_total}");
}


