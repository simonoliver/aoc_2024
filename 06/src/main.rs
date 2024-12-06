
const MOVE_DIRECTIONS:[(i32, i32);4]=[(-1, 0),(0, 1),(1, 0),(0, -1)]; // Move directions. Up, right, down, left
fn find_first_char(grid_data : &Vec<Vec<char>>,search_char : char) -> (bool,i32,i32)
{
    for (row_index,row) in grid_data.iter().enumerate() {
        for (column_index,test_char) in row.iter().enumerate() {
            if *test_char==search_char {return (true,row_index as i32,column_index as i32)}
        }
    }
    (false,0,0)
}

fn process_agent_step(grid_data : &Vec<Vec<char>>, agent_state :&mut (i32,i32,i32)) -> (bool,i32)
{
    let step:(i32,i32)=MOVE_DIRECTIONS[agent_state.2 as usize];
    let next_step=(agent_state.0+step.0,agent_state.1+step.1,0);
    //agent_state.0+=step.0; agent_state.1+=step.1; // Neater way of doing this?
    if next_step.0<0 || next_step.0>=(grid_data.len() as i32) // Row bounds check
        || next_step.1<0 || next_step.1>=(grid_data[0].len() as i32) {return (false,0)} // Column Bounds check

    println!("Char at Row{} Column{} is {} Direction {}",next_step.0,next_step.1,grid_data[next_step.0 as usize][next_step.1 as usize],agent_state.2);
    if grid_data[next_step.0 as usize][next_step.1 as usize]=='#'
        {agent_state.2=(agent_state.2+1)%4;(true,0)} // Rotate right, no step
    else
        {agent_state.0=next_step.0;agent_state.1=next_step.1;(true,1)} // Move into next pos, valid step
}

fn main() {

    let contents=std::fs::read_to_string("data/test_input").expect("Should be able to load");
    let lines=contents.split("\n");
    let char_lines:Vec<Vec<char>>=lines.filter(|line|line.len()>0).into_iter().map(|line|line.chars().collect()).collect(); // Prase grid
    let (_,agent_row,agent_column)=find_first_char(&char_lines,'^');
    println!("Start Pos Row {agent_row} Column {agent_column},");
    let mut agent_state=(agent_row,agent_column,0); // Row/column/direction (0=up,1=right,2=down,3=left)
    let mut steps=0;
    let mut step_position_indices:Vec<i32>=Vec::new(); // All step positions
    step_position_indices.push(agent_row*char_lines.len() as i32+agent_column); // Add start position
    loop {
        let (valid_move,step_count)=process_agent_step(&char_lines, &mut agent_state);
        steps+=step_count;
        let step_position_index=agent_state.0*(char_lines.len() as i32)+agent_state.1;
        if !step_position_indices.contains(&step_position_index) {step_position_indices.push(step_position_index);}
        if !valid_move {break}
    }
    println!("Pt1 - Guard steps {steps} unique positions {}",step_position_indices.len());
}
