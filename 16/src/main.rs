use std::fs;
use grid::*; //https://docs.rs/grid/latest/grid/

enum GridEntry {
    Empty, Block, StartPosition, EndPosition
}
fn print_map(map:Grid<GridEntry>) {
    for row in map.iter_rows() {
        println!("{}",row.fold(String::new(),|mut acc, entry| {
            match entry {
                GridEntry::Empty => { acc.push('.'); }
                GridEntry::Block => { acc.push('#'); }
                GridEntry::StartPosition => { acc.push('S'); }
                GridEntry::EndPosition => { acc.push('E'); }
            };
            acc
        }));
    }
}

fn pos_from_index(index:i32,grid_width:i32) -> (i32,i32) {
    (index%grid_width, ((index as f64/grid_width as f64).floor()) as i32)
}

fn parse_map(content:&str,line_length:usize) -> (Grid<GridEntry>,(i32,i32),(i32,i32)) {
    let (mut start_pos,mut end_pos)=((0,0),(0,0));
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
    let content = fs::read_to_string("data/test_input").expect("Expected to read the file");
    let lines:Vec<&str>=content.split("\n").filter(|line|line.len()>0).collect();
    let line_length=lines[0].len();
    let (grid,start_pos,end_pos)=parse_map(&content,line_length);
    print_map(grid);
}
