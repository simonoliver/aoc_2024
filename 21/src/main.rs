use std::collections::HashMap;
use std::{cmp, fs};
use grid::Grid;

const DIRECTION_CHARS:[char;4]=['<','>','^','v'];

struct Keypad {
    grid: Grid<char>,
    entry_position_table: HashMap<char,(usize,usize)>, // Row/column
}

impl Keypad {

    fn get_move_sequence_for_output(self:&Keypad,required_output:Vec<char>,position:(usize,usize)) -> Vec<char> {
        let mut sequence:Vec<char>=Vec::new();
        let mut current_position=position;
        for char in required_output {
            let target_pos=self.entry_position_table.get(&char).unwrap();
            let diff=(target_pos.0 as i32 - current_position.0 as i32,target_pos.1 as i32 - current_position.1 as i32);

            let direction_count=vec!{cmp::max(-1* diff.1 as i32,0),cmp::max(diff.1,0) as i32,cmp::max(-1* diff.0 as i32,0),cmp::max(diff.0,0) as i32}; //L/R/U/D
            println!("Move from {:?} to {:?} diff is {:?} direction count {:?}",current_position,target_pos,diff,direction_count);
            for (index,count) in direction_count.iter().enumerate() {
                for i in 0..*count {
                    sequence.push(DIRECTION_CHARS[index]); // Add direction
                }
            }
            sequence.push('A'); // Add "Press" button
            current_position=*target_pos; // Now at target pos
        }
        sequence
    }

    fn new_from_vec(input:Vec<char>,width:usize) -> Self {
        let grid=Grid::from_vec(input,width);
        let mut entry_position_table:HashMap<char,(usize,usize)>=HashMap::new();
        for grid_entry in grid.indexed_iter() { // Record each
            //println!("Registering {} at {:?}",grid_entry.1,(grid_entry.0.0,grid_entry.0.1));
            entry_position_table.entry(*grid_entry.1).or_insert((grid_entry.0.0,grid_entry.0.1)); // Row, column
        }
        Self {
            grid:grid,
            entry_position_table:entry_position_table
        }
    }
}

fn main() {
    //solve("data/input");
    solve("data/test_input");
}

fn solve(data_path:&str)
{
    let content = fs::read_to_string(data_path).expect("Expected to read the file");
    let lines=content.split("\n").filter(|line|!line.is_empty());
    let num_keypad=Keypad::new_from_vec(vec!{'7','8','9','4','5','6','1','2','3',' ','0','A'},3);
    let dir_keypad=Keypad::new_from_vec(vec!{' ','^','A','<','v','>'},3);
    for line in lines {
        let sequence_keypad=num_keypad.get_move_sequence_for_output(line.chars().collect(),(3,2)); // Row, column
        let sequence_dir_pad_0=dir_keypad.get_move_sequence_for_output(sequence_keypad,(0,2));
        let sequence_dir_pad_1=dir_keypad.get_move_sequence_for_output(sequence_dir_pad_0,(0,2));

        let output_str_keypad=sequence_dir_pad_1.iter().fold(String::new(),|mut acc,entry|{acc.push(*entry);acc});
        println!("Output for {} is {:?} length {}",line,&output_str_keypad,output_str_keypad.len());
    }

}
