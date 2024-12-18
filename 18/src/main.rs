use std::collections::HashMap;
use std::fs;
use grid::*; //https://docs.rs/grid/latest/grid/
use pathfinding::prelude::{astar, astar_bag}; // https://docs.rs/pathfinding/latest/pathfinding/directed/astar/fn.astar.html
use colored::*;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct GridPos(i32,i32,u8); // x,y,rotation
const DIRECTIONS:[(i32,i32);4]=[(1,0),(0,1),(-1,0),(0,-1)];
const DIRECTION_SYMBOLS:[char;4]=['→','↓','←','↑'];

enum GridEntry {
    Empty, Block, StartPosition, EndPosition
}
impl Default for GridEntry {
    fn default() -> Self { GridEntry::Empty }
}

impl GridPos {
    fn distance(&self, other: &GridPos) -> u32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32 // Manhattan distance
    }
    fn successors(&self, grid: &Grid<GridEntry>,drop_pos_map:&HashMap<(i32,i32),i32>,drop_pos_advance:i32) -> Vec<(GridPos, u32)> {
        // Add all valid neighbors as successors
        let grid_size=grid.size();
        let mut valid_successors: Vec<(GridPos, u32)> =Vec::new();
        for (direction_index,direction) in DIRECTIONS.iter().enumerate() {
            let test_pos=(self.0+direction.0,self.1+direction.1);
            if test_pos.0>=0 && test_pos.0<grid_size.0 as i32 && test_pos.1>=0 && test_pos.1<grid_size.1 as i32 { // Bounds
                // Blocked if there is an entry and its blocked
                let blocked=drop_pos_map.contains_key(&test_pos) && drop_pos_map[&test_pos]<drop_pos_advance;
                if !blocked {
                    valid_successors.push((GridPos(test_pos.0,test_pos.1,direction_index as u8),{1})); // Increment step
                }
                /*
                match grid.get(test_pos.1,test_pos.0) {
                    None => {}
                    Some(entry) => { match entry {
                        GridEntry::Block => {},
                        _ => {valid_successors.push((GridPos(test_pos.0,test_pos.1,direction_index as u8,self.3+1),{1}));} // Increment step
                    };}
                };
                */
            }
        }
        valid_successors
    }
}

fn print_map(map:&Grid<GridEntry>, path:&Vec<GridPos>,drop_pos_map:&HashMap<(i32,i32),i32>,drop_pos_advance:i32) {
    let mut map_directions:HashMap<(i32,i32),char>=HashMap::new();
    for path_entry in path {
        map_directions.entry((path_entry.0,path_entry.1)).or_insert(DIRECTION_SYMBOLS[path_entry.2 as usize]);
    }
    for (row_index,row) in map.iter_rows().enumerate() {
        let mut char_index=0;
        let row_string=row.fold(String::new(),|mut acc, entry| {
            if map_directions.contains_key(&(char_index,row_index as i32)) {
                acc.push_str(map_directions[&(char_index,row_index as i32)].to_string().green().to_string().as_str());
            }
            else if drop_pos_map.contains_key(&(char_index,row_index as i32)) && drop_pos_map[&(char_index,row_index as i32)]<drop_pos_advance {
                acc.push_str("▓".bright_black().to_string().as_str());
            }
            else {
                match entry {
                    GridEntry::Empty => { acc.push_str(".".blue().to_string().as_str()); }
                    GridEntry::Block => { acc.push_str("▓".bright_black().to_string().as_str()); }
                    GridEntry::StartPosition => { acc.push('S'); }
                    GridEntry::EndPosition => { acc.push('E'); }
                };
            }
            char_index+=1;
            acc
        });
        println!("{}",row_string);
    }
}

fn main() {
    solve("data/input",(71,71),1024);
    //solve("data/test_input",(7,7),12);
}

fn solve(input_path:&str,grid_size:(usize,usize),advance_count:i32) {

    let content = fs::read_to_string(input_path).expect("Expected to read the file");
    let lines:Vec<&str>=content.split("\n").filter(|line|line.len()>0).collect();
    let drop_sequence:Vec<(i32,i32)>=lines.iter().map(
        |pair_string|{
            let mut pair_split=pair_string.split(",");
            (pair_split.next().unwrap().parse::<i32>().unwrap(),pair_split.next().unwrap().parse::<i32>().unwrap())
        }).collect();
    let mut drop_pos_map:HashMap<(i32,i32),i32>=HashMap::new();
    for (index,drop_pair) in drop_sequence.iter().enumerate() {
        println!("{},{}",drop_pair.0,drop_pair.1);
        drop_pos_map.insert((drop_pair.0,drop_pair.1),index as i32);
    }
    let grid:Grid<GridEntry>=Grid::new(grid_size.1,grid_size.0);

    let start_pos=GridPos(0,0,0);
    let end_pos=GridPos((grid_size.1-1) as i32,(grid_size.1-1) as i32,0);
    let pos_list:Vec<GridPos>=Vec::new();
    print_map(&grid,&pos_list,&drop_pos_map,advance_count);

    let result = astar(&start_pos,
                       |test_pos| test_pos.successors(&grid,&drop_pos_map,advance_count),
                       |test_pos| test_pos.distance(&end_pos) / 3,
                       |test_pos| (test_pos.0,test_pos.1) == (end_pos.0,end_pos.1));

    match result {
        None => {println!("Unable to find path")},
        Some(path_result) => {
            print_map(&grid,&path_result.0,&drop_pos_map,advance_count);
            println!("Pt1 - Step count {}",path_result.0.len()-1); // Ignore first location
        }
    }

    // Pt2
    let mut advance_count_inc=advance_count;
    loop {
        let result = astar(&start_pos,
                           |test_pos| test_pos.successors(&grid,&drop_pos_map,advance_count_inc),
                           |test_pos| test_pos.distance(&end_pos) / 3,
                           |test_pos| (test_pos.0,test_pos.1) == (end_pos.0,end_pos.1));

        match result {
            None => {
                let block_pos=drop_sequence[(advance_count_inc-1) as usize];
                println!("pt2 - blocked at index {} - Pos is {},{}",advance_count_inc,block_pos.0,block_pos.1);break;
            },
            Some(path_result) => {
                //println!("Loop count pass {advance_count_inc}");
                //print_map(&grid,&path_result.0,&drop_pos_map,advance_count);
                //println!("Pt1 - Step count {}",path_result.0.len()-1); // Ignore first location
            }
        }
        advance_count_inc+=1;
    }


}
