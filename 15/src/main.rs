use std::fs;

enum GridEntryType {
    Empty,
    Block,
    Box
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

// Find spaces in direction
fn find_movable_count_in_direction(map: &mut Vec<Vec<GridEntryType>>,pos:(i64,i64),direction_step:(i64,i64)) -> (bool,i64)
{
    let mut count=0;
    let grid_size=(map[0].len() as i64,map.len() as i64);
    loop
    {
        count+=1;
        let test_pos=(pos.0+direction_step.0*count,pos.1+direction_step.1*count);
        if test_pos.0<0 || test_pos.0>=grid_size.0 || test_pos.1<0 || test_pos.1>=grid_size.1 { // Bounds check
            return (false,0);
        }
        match map[test_pos.1 as usize][test_pos.0 as usize] {
            GridEntryType::Block =>  {return (false,0)},
            GridEntryType::Box => {},
            GridEntryType::Empty => {return (true,count)},
        }
    }
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
                    GridEntryType::Box => { string_line.push('O'); }
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

fn main() {
    let content = fs::read_to_string("data/test_input").expect("Expected to read the file");
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
    println!("Line split at line {}",section_split_index.0);
    for (line_index,line) in lines.enumerate() {
        if line_index<section_split_index.0 {map_lines.push(line)}
        else if line_index>section_split_index.0 {directions_lines.push(line)};
    }
    let (mut map,mut agent_pos)=parse_map(&map_lines);
    let agent_directions_sequence=parse_directions(&directions_lines);
    println!("Line split at line {} map lines {} directions {} start pos {},{}",section_split_index.0,map.len(),agent_directions_sequence.len(),agent_pos.0,agent_pos.1);

    print_map(&map,&agent_pos);

    for direction in agent_directions_sequence {
        process_direction(&mut map,&mut agent_pos,direction);
    }
    print_map(&map,&agent_pos);
}




