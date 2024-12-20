use std::collections::HashMap;
use std::fs;
use grid::*; //https://docs.rs/grid/latest/grid/
use pathfinding::prelude::astar; // https://docs.rs/pathfinding/latest/pathfinding/directed/astar/fn.astar.html
use colored::*;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct GridPos(i32,i32,u8); // x,y,rotation

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum GridEntry {
    Empty, Block, StartPosition, EndPosition
}

const DIRECTIONS:[(i32,i32);4]=[(1,0),(0,1),(-1,0),(0,-1)];
const DIRECTION_SYMBOLS:[char;4]=['→','↓','←','↑'];
impl GridPos {
    fn distance(&self, other: &GridPos) -> u32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32 // Manhattan distance
    }
    fn successors(&self, grid: &Grid<GridEntry>,skip_pos:(bool,i32,i32)) -> Vec<(GridPos, u32)> {
        // Add all valid neighbors as successors
        let grid_size=grid.size();
        let mut valid_successors: Vec<(GridPos, u32)> =Vec::new();
        for (direction_index,direction) in DIRECTIONS.iter().enumerate() {
            let test_pos=(self.0+direction.0,self.1+direction.1);
            if test_pos.0>=0 && test_pos.0<grid_size.0 as i32 && test_pos.1>=0 && test_pos.1<grid_size.1 as i32 {
                if skip_pos.0 && skip_pos.1==test_pos.0 && skip_pos.2==test_pos.1 {
                    // Skipped position
                    valid_successors.push((GridPos(test_pos.0, test_pos.1, direction_index as u8), { 1 }));
                } else {
                match grid.get(test_pos.1,test_pos.0) {
                    None => {}
                    Some(entry) => { match entry {
                        GridEntry::Block => {},
                        _ => {valid_successors.push((GridPos(test_pos.0,test_pos.1,direction_index as u8),{1}));} // Cost is 1001 if turning
                    };}
                };
                    }
            }
        }
        valid_successors
    }
}


fn print_map(map:&Grid<GridEntry>, path:&Vec<GridPos>) {
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

fn pos_from_index(index:i32,grid_width:i32) -> GridPos {
    GridPos(index%grid_width, (index as f64/grid_width as f64).floor() as i32, 0)
}

fn parse_map(content:&str,line_length:usize) -> (Grid<GridEntry>,GridPos,GridPos) {
    let (mut start_pos,mut end_pos)=(GridPos(0,0,0),GridPos(0,0,0));
    let entries:Vec<GridEntry>=content.chars().fold(Vec::new(),  |mut acc,entry_char| {match entry_char {
        '#' => acc.push(GridEntry::Block),
        'S' => {start_pos=pos_from_index(acc.len() as i32,line_length as i32);acc.push(GridEntry::StartPosition)},
        'E' => {end_pos=pos_from_index(acc.len() as i32,line_length as i32);acc.push(GridEntry::EndPosition)},
        '.' => acc.push(GridEntry::Empty),
        _ => {}
    };acc});
    (Grid::from_vec(entries,line_length),start_pos,end_pos)
}

fn main() {
    solve("data/input");
    //solve("data/test_input");
}

fn get_score_for_map_modification(grid:&mut Grid<GridEntry>,skip_pos:(bool,i32,i32),start_pos:&GridPos,end_pos:&GridPos) -> (bool,u32) {

    /*
    //let mut current_grid_value:GridEntry=GridEntry::Block;
    if grid_modification.0 { // If we are modifying
        //current_grid_value=grid.get(grid_modification.2,grid_modification.1).unwrap().clone();
       let mut grid_entry=grid.get_mut(grid_modification.2, grid_modification.1).unwrap();
        match (grid_entry) {
            GridEntry::Block(mut block_val) => block_val=false,
            _ => {return (false,0,0)}
        }
        println!("Replacing at {},{} previous val was {:?} Now {:?}",grid_modification.1,grid_modification.2,current_grid_value,grid.get(grid_modification.2,grid_modification.1).unwrap().clone());
    }
    */

    let result = astar(start_pos,
                       |test_pos| test_pos.successors(&grid,skip_pos),
                       |test_pos| test_pos.distance(end_pos) / 3,
                       |test_pos| (test_pos.0,test_pos.1) == (end_pos.0,end_pos.1));

    /*
    if grid_modification.0 { // Swap back for future ref
        grid.get(grid_modification.2, grid_modification.1).replace(&current_grid_value);
    }
    */
    match result {
        None => {(false,0)},
        Some(path_result) => {
            //println!("Solution : Path length {}, cost {}",path_result.0.len(),path_result.1);
            (true,path_result.1)
            //print_map(&grid,&path_result.0);
        }
    }
}
fn solve(data_path:&str)
{
    let content = fs::read_to_string(data_path).expect("Expected to read the file");
    let lines:Vec<&str>=content.split("\n").filter(|line|line.len()>0).collect();
    let line_length=lines[0].len();
    let (mut grid,start_pos,end_pos)=parse_map(&content,line_length);
    print_map(&grid,&Vec::new());

    let base_cost=get_score_for_map_modification(&mut grid,(false,0,0),&start_pos,&end_pos);
    println!("Base cost {}",base_cost.1);

    let mut cheat_map:HashMap<u32,u32>=HashMap::new();
    let grid_size=grid.size();
    let mut over_100_count=0;
    for row_index in 0..grid_size.0 {
        for column_index in 0..grid_size.1 {
            let grid_value= grid.get(row_index,column_index).unwrap().clone();
            if grid_value==GridEntry::Block {
                let mod_score=get_score_for_map_modification(&mut grid,(true,column_index as i32,row_index as i32),&start_pos,&end_pos);
                if mod_score.0 && mod_score.1<base_cost.1 {
                    let cheat_save= base_cost.1-mod_score.1;
                    cheat_map.entry(cheat_save).and_modify(|value|{*value+=1;}).or_insert(1);
                    if cheat_save>=100 {over_100_count+=1;}
                    //if !cheat_map.contains_key()
                    println!("Cheat at {},{} - new val {} saves {}",column_index,row_index,mod_score.1,cheat_save);
                }
            }
        }
    }

    for pair in cheat_map {
        println!("Cheat amount {} Count {}",pair.0,pair.1);
    }
    println!("pt1 - Over 100 count {over_100_count}");
}
