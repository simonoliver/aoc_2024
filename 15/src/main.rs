use std::fs;
use std::io;
use std::io::prelude::*;

#[derive(Copy,Clone,Debug)]
enum GridEntryType {
    Empty,
    Block,
    Box,
    LargeBoxLeft,
    LargeBoxRight
}

const DIRECTION_CHARS:[char;4]=['>','v','<','^'];
const DIRECTION_MOVEMENT:[(i64,i64);4]=[(1,0),(0,1),(-1,0),(0,-1)];

fn parse_map(lines:&Vec<&str>) -> (Vec<Vec<GridEntryType>>,(i64,i64)) {
    let mut grid:Vec<Vec<GridEntryType>> = Vec::new();
    let mut start_pos:(i64,i64)=(0,0);
    for line in lines {
        grid.push(line.chars().fold(Vec::new(),|mut acc,char_entry|{
            acc.push(
            match char_entry {
                '#' => GridEntryType::Block,
                'O' => GridEntryType::Box,
                '@' => {start_pos=(acc.len() as i64,grid.len() as i64);GridEntryType::Empty},
                _ => GridEntryType::Empty
            });
            acc })
        )
    };
    (grid,start_pos)
}

fn parse_map_wide(lines:&Vec<&str>) -> (Vec<Vec<GridEntryType>>,(i64,i64)) {
    let mut grid:Vec<Vec<GridEntryType>> = Vec::new();
    let mut start_pos:(i64,i64)=(0,0);
    for line in lines {
        grid.push(line.chars().fold(Vec::new(),|mut acc,char_entry|{
            acc.extend( // Extend rather than push
                match char_entry {
                    '#' => vec!{GridEntryType::Block,GridEntryType::Block},
                    'O' => vec!{GridEntryType::LargeBoxLeft,GridEntryType::LargeBoxRight},
                    '@' => {start_pos=(acc.len() as i64,grid.len() as i64);vec!{GridEntryType::Empty,GridEntryType::Empty}},
                    _ => vec!{GridEntryType::Empty,GridEntryType::Empty}
                });
            acc })
        )
    };
    (grid,start_pos)
}

// Find spaces in direction
fn find_movable_count_in_direction(map: &mut Vec<Vec<GridEntryType>>,pos:(i64,i64),direction_step:(i64,i64)) -> (bool,i64)
{
    let mut count=0;
    let grid_size=(map[0].len() as i64,map.len() as i64);
    loop {
        count+=1;
        let test_pos=(pos.0+direction_step.0*count,pos.1+direction_step.1*count);
        if test_pos.0<0 || test_pos.0>=grid_size.0 || test_pos.1<0 || test_pos.1>=grid_size.1 { // Bounds check
            return (false,0);
        }
        match map[test_pos.1 as usize][test_pos.0 as usize] {
            GridEntryType::Block =>  {return (false,0)},
            GridEntryType::Box => {},
            GridEntryType::Empty => {return (true,count)},
            _ => {}
        }
    }
}


fn test_bounds(map: &Vec<Vec<GridEntryType>>,test_pos: (i64, i64)) -> bool
{
    let grid_size=(map[0].len() as i64,map.len() as i64);
    if test_pos.0<0 || test_pos.0>=grid_size.0 || test_pos.1<0 || test_pos.1>=grid_size.1 {
        return false;
    }
    true
}

fn get_movable_blocks_in_direction(map: &Vec<Vec<GridEntryType>>,test_pos: (i64, i64),direction: (i64, i64),movable_block_positions:&mut Vec<(i64,i64)>) {
    if !test_bounds(map,test_pos) {return;}

    let (mut add_pos,mut extra_pos)=match map[test_pos.1 as usize][test_pos.0 as usize] {
        GridEntryType::LargeBoxLeft => {(true,(test_pos.0+1,test_pos.1))},
        GridEntryType::LargeBoxRight => {(true,(test_pos.0-1,test_pos.1))},
        _ => {(false,(0,0))},
    };
    if add_pos {
        if !movable_block_positions.contains(&test_pos) {
            movable_block_positions.push(test_pos);
            get_movable_blocks_in_direction(map,(test_pos.0+direction.0,test_pos.1+direction.1),direction,movable_block_positions);
        }
        if !movable_block_positions.contains(&extra_pos) {
            movable_block_positions.push(extra_pos);
            get_movable_blocks_in_direction(map,(extra_pos.0+direction.0,extra_pos.1+direction.1),direction,movable_block_positions);
        }
    }
}

fn try_movable_blocks_can_move_in_direction(map: &mut Vec<Vec<GridEntryType>>,movable_block_positions:&Vec<(i64,i64)>,direction: (i64, i64)) -> bool {
    // Test all can move

    // Cache write positions
    let mut write_data:Vec<(GridEntryType,(i64,i64))>=Vec::new();

    for block_pos in movable_block_positions {
        let move_to_pos=(block_pos.0+direction.0,block_pos.1+direction.1);
        if !test_bounds(map,move_to_pos) {return false;}
        match map[move_to_pos.1 as usize][move_to_pos.0 as usize] {
            GridEntryType::Block=> {return false;} // Cannot push this into a block
            _ => {}
        }
        write_data.push((map[block_pos.1 as usize][block_pos.0 as usize],*block_pos));
    }
    // Erase all first
    for write_src_pos in &write_data {
        map[write_src_pos.1.1 as usize][write_src_pos.1.0 as usize] = GridEntryType::Empty; // erase old position
    }
    // Then write
    for write_src_pos in &write_data {
        let move_to_pos = (write_src_pos.1.0 + direction.0, write_src_pos.1.1 + direction.1);
        map[move_to_pos.1 as usize][move_to_pos.0 as usize]=write_src_pos.0; // Write new data
    }
    true
}

fn process_direction_wide(map: &mut Vec<Vec<GridEntryType>>, agent_pos: &mut (i64, i64), direction: u8)
{
    let direction_step=DIRECTION_MOVEMENT[direction as usize];
    let mut movable_block_positions:Vec<(i64,i64)>=Vec::new();
    get_movable_blocks_in_direction(map,(agent_pos.0+direction_step.0,agent_pos.1+direction_step.1),direction_step,&mut movable_block_positions);
    let mut move_player=false;
    let move_player_step=(agent_pos.0+direction_step.0,agent_pos.1+direction_step.1);
    if movable_block_positions.len()==0 {
        move_player=match map[move_player_step.1 as usize][move_player_step.0 as usize] {
            GridEntryType::Empty => true,
            _ => false
        }
    }
    else {
        move_player=try_movable_blocks_can_move_in_direction(map,&mut movable_block_positions,direction_step);
    }
    if move_player {agent_pos.0=move_player_step.0;agent_pos.1=move_player_step.1;}
}

fn process_direction(map: &mut Vec<Vec<GridEntryType>>, agent_pos: &mut (i64, i64), direction: u8) {
    let direction_step=DIRECTION_MOVEMENT[direction as usize];
    let (can_move,step_count) =find_movable_count_in_direction(map,*agent_pos,direction_step);
    if can_move {
        // Shift blocks as needed
        for step_index in 0..step_count-1 {
            let step_index=step_count-step_index;
            let to_pos=(agent_pos.0+direction_step.0*step_index,agent_pos.1+direction_step.1*step_index);
            let from_pos=(agent_pos.0+direction_step.0*(step_index-1),agent_pos.1+direction_step.1*(step_index-1));
            //let copy_val=(&map[from_pos.1 as usize][from_pos.0 as usize]);
            map[to_pos.1 as usize][to_pos.0 as usize]=GridEntryType::Box;
            map[from_pos.1 as usize][from_pos.0 as usize]=GridEntryType::Empty;
        }
        // move
        agent_pos.0 += direction_step.0;
        agent_pos.1 += direction_step.1;
    }
}

fn calculate_coordinates(grid:&Vec<Vec<GridEntryType>>) -> i64 {
    let mut coord_total:i64=0;
    for (row_index,row) in grid.iter().enumerate() {
        for (column_index,entry_value) in row.iter().enumerate() {
            match entry_value
            {
                GridEntryType::Box | GridEntryType::LargeBoxLeft=>  {coord_total+=row_index as i64*100+column_index as i64;}
                _ => {}
            }
        }
    }
    coord_total
}

fn print_map(grid:&Vec<Vec<GridEntryType>>,agent_pos:&(i64,i64)) {
    for (row_index,grid_line) in grid.iter().enumerate() {
        let mut string_line=String::new();
        for (column_index,entry) in grid_line.iter().enumerate() {
            if (column_index as i64,row_index as i64)==*agent_pos {
                string_line.push('@');
            } else {
                match entry {
                    GridEntryType::Empty => { string_line.push('.'); }
                    GridEntryType::Block => { string_line.push('#'); }
                    GridEntryType::Box => { string_line.push('O'); },
                    GridEntryType::LargeBoxLeft => { string_line.push('[');},
                    GridEntryType::LargeBoxRight => { string_line.push(']');},
                }
            }
        }
        println!("{string_line}");
    }
}

fn parse_directions(lines:&Vec<&str>) -> Vec<u8> {
    let mut directions:Vec<u8>=Vec::new();
    for line in lines {
        for char in line.chars() {
            let char_option= DIRECTION_CHARS.iter().position(|test_char| *test_char==char);
            if let Some(index) = char_option {directions.push(index as u8)}
        }
    }
    directions
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn main() {
    let content = fs::read_to_string("data/input").expect("Expected to read the file");
    let lines=content.split("\n");
    let section_split_index=lines.clone().fold((0,false),|mut acc,line|
        {
            if !acc.1 { // Not found yet
                if line.len() == 0 { acc.1 = true; } else { acc.0 += 1; }
            };
            acc
        }
    );
    let mut map_lines:Vec<&str>=Vec::new();
    let mut directions_lines:Vec<&str>=Vec::new();

    for (line_index,line) in lines.enumerate() {
        if line_index<section_split_index.0 {map_lines.push(line)}
        else if line_index>section_split_index.0 {directions_lines.push(line)};
    }
    let (mut map,mut agent_pos)=parse_map(&map_lines);
    let (mut map_wide,mut agent_pos_wide)=parse_map_wide(&map_lines);
    let agent_directions_sequence=parse_directions(&directions_lines);

    print_map(&map,&agent_pos);

    for direction in &agent_directions_sequence {
        process_direction(&mut map,&mut agent_pos,*direction);
    }
    print_map(&map,&agent_pos);
    println!("Pt1 - {}",calculate_coordinates(&map));

    print_map(&map_wide,&agent_pos_wide);
    for direction in &agent_directions_sequence {
        process_direction_wide(&mut map_wide,&mut agent_pos_wide,*direction);
    }
    print_map(&map_wide,&agent_pos_wide);
    println!("Pt2 - {}",calculate_coordinates(&map_wide));
}
