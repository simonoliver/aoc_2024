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
    let (map,start_pos)=parse_map(&map_lines);

    println!("Line split at line {} map lines {} directions {} start pos {},{}",section_split_index.0,map.len(),directions_lines.len(),start_pos.0,start_pos.1);
}



